//! Our changes to the iROSE game data, kept in one file (`overrides/game-data.toml`) instead
//! of edited copies of the client files. Each entry sets one cell of an STB table. The client
//! applies them when it loads its data, and the server's upload (`rose-stdb-import pack`)
//! applies them to the files it sends, so both always see the same values.

use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
};

use anyhow::Context;
use rose_file_readers::{patch_stb, MemoryFilesystemDevice, VfsPath, VirtualFilesystem};
use serde::Deserialize;

/// The file both sides look for, relative to the repository (and to the game's folder in
/// the Windows zip).
pub const OVERRIDES_FILE: &str = "overrides/game-data.toml";

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Overrides {
    #[serde(default)]
    pub stb: Vec<StbCell>,
}

/// One cell: `row` and `column` are numbered as the data loaders read them (row = the item,
/// skill or NPC number; column 0 = the first column after the row name).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StbCell {
    pub file: String,
    pub row: usize,
    pub column: usize,
    pub value: CellValue,
    /// What the change is and which wiki page asks for it.
    #[serde(default)]
    pub note: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CellValue {
    Int(i64),
    Float(f64),
    Text(String),
}

impl CellValue {
    pub fn text(&self) -> String {
        match self {
            CellValue::Int(v) => v.to_string(),
            CellValue::Float(v) => v.to_string(),
            CellValue::Text(v) => v.clone(),
        }
    }
}

impl Overrides {
    pub fn parse(text: &str) -> Result<Self, anyhow::Error> {
        Ok(toml::from_str(text)?)
    }

    /// Read the file; a missing file means no changes.
    pub fn load(path: &Path) -> Result<Self, anyhow::Error> {
        match std::fs::read_to_string(path) {
            Ok(text) => Self::parse(&text).with_context(|| format!("reading {}", path.display())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(e) => Err(e).with_context(|| format!("reading {}", path.display())),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stb.is_empty()
    }

    /// The changed cells per file, by normalised VFS path.
    fn by_file(&self) -> BTreeMap<PathBuf, Vec<(usize, usize, String)>> {
        let mut out: BTreeMap<PathBuf, Vec<(usize, usize, String)>> = BTreeMap::new();
        for cell in &self.stb {
            out.entry(VfsPath::normalise_path(&cell.file)).or_default().push((cell.row, cell.column, cell.value.text()));
        }
        out
    }

    /// Patch files already read into memory (the server upload), keyed by normalised VFS path.
    pub fn patch_files(&self, files: &mut BTreeMap<PathBuf, Vec<u8>>) -> Result<usize, anyhow::Error> {
        let mut count = 0;
        for (path, cells) in self.by_file() {
            let data = files.get_mut(&path).with_context(|| format!("{} is not game data the server reads", path.display()))?;
            *data = patch_stb(data, &cells).with_context(|| format!("patching {}", path.display()))?;
            count += cells.len();
        }
        Ok(count)
    }

    /// Put the patched files in front of the file system's other devices (the client).
    pub fn apply(&self, vfs: &mut VirtualFilesystem) -> Result<usize, anyhow::Error> {
        if self.is_empty() {
            return Ok(0);
        }
        let mut patched = HashMap::new();
        let mut count = 0;
        for (path, cells) in self.by_file() {
            let name = path.to_string_lossy().to_string();
            let file = vfs.open_file(name.as_str()).with_context(|| format!("opening {name}"))?;
            let bytes: &[u8] = match &file {
                rose_file_readers::VfsFile::Buffer(b) => b,
                rose_file_readers::VfsFile::View(v) => v,
            };
            patched.insert(path, patch_stb(bytes, &cells).with_context(|| format!("patching {name}"))?);
            count += cells.len();
        }
        let mut device = MemoryFilesystemDevice::default();
        device.files = patched;
        vfs.devices.insert(0, Box::new(device));
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rose_file_readers::{RoseFile, RoseFileReader, StbFile};

    /// A 2 x 2 STB file (plus the header row and the row-name column).
    fn stb(cells: [&str; 4]) -> Vec<u8> {
        let mut head = Vec::new();
        head.extend_from_slice(b"STB1");
        let mut names = Vec::new();
        for name in ["", "a", "b"] {
            names.extend_from_slice(&(name.len() as u16).to_le_bytes());
            names.extend_from_slice(name.as_bytes());
        }
        let mut body = Vec::new();
        for c in ["", "r1", "r2"] {
            body.extend_from_slice(&(c.len() as u16).to_le_bytes());
            body.extend_from_slice(c.as_bytes());
        }
        let header_len = 4 + 4 * 4 + 2 * 4;
        let data_position = header_len + names.len() + 2 + body.len();
        head.extend_from_slice(&(data_position as u32).to_le_bytes());
        head.extend_from_slice(&3u32.to_le_bytes()); // rows with the header row
        head.extend_from_slice(&3u32.to_le_bytes()); // columns with the row names
        head.extend_from_slice(&0u32.to_le_bytes()); // row height
        head.extend_from_slice(&[0u8; 8]); // column widths
        head.extend_from_slice(&names);
        head.extend_from_slice(&0u16.to_le_bytes()); // column title line
        head.extend_from_slice(&body[2..]); // row names after the empty title
        head.extend_from_slice(&0u16.to_le_bytes());
        assert_eq!(head.len(), data_position);
        for c in cells {
            head.extend_from_slice(&(c.len() as u16).to_le_bytes());
            head.extend_from_slice(c.as_bytes());
        }
        head
    }

    fn read(bytes: &[u8]) -> StbFile {
        StbFile::read(RoseFileReader::from(bytes), &Default::default()).unwrap()
    }

    #[test]
    fn changes_one_cell_and_keeps_the_rest() {
        let file = stb(["1", "22", "333", "4"]);
        assert_eq!(read(&file).get(1, 0), "333");
        let overrides = Overrides::parse(
            r#"
            [[stb]]
            file = "3DDATA/STB/TEST.STB"
            row = 1
            column = 0
            value = 12
            note = "test"
            "#,
        )
        .unwrap();
        let mut files = BTreeMap::new();
        files.insert(VfsPath::normalise_path("3ddata\\stb\\test.stb"), file);
        assert_eq!(overrides.patch_files(&mut files).unwrap(), 1);
        let patched = read(files.values().next().unwrap());
        assert_eq!(patched.get(0, 0), "1");
        assert_eq!(patched.get(0, 1), "22");
        assert_eq!(patched.get(1, 0), "12");
        assert_eq!(patched.get(1, 1), "4");
        assert_eq!(patched.get_row_name(1), "r2");
    }

    #[test]
    fn refuses_cells_outside_the_table() {
        let overrides =
            Overrides::parse("[[stb]]\nfile = \"T.STB\"\nrow = 5\ncolumn = 0\nvalue = \"x\"\n").unwrap();
        let mut files = BTreeMap::new();
        files.insert(VfsPath::normalise_path("T.STB"), stb(["1", "2", "3", "4"]));
        assert!(overrides.patch_files(&mut files).is_err());
    }

    #[test]
    fn unknown_keys_are_an_error() {
        assert!(Overrides::parse("[[stb]]\nfile = \"T.STB\"\nrow = 1\ncolumn = 0\nvalue = 1\ncolum = 2\n").is_err());
    }
}
