//! `export/motions.toml`: how long each player, monster and NPC motion is and on which frames it hits,
//! exported from the hit markers of the Godot animations (`--export-data`). The server's
//! game data uses these numbers in place of the ones in the ZMO files, so moving a hit
//! marker in the Godot editor moves the hit on the server too.

use std::{path::Path, time::Duration};

use anyhow::Context;
use serde::Deserialize;

/// The file in the repository (and in the export folder on a server).
pub const MOTIONS_FILE: &str = "export/motions.toml";
/// Where the upload puts it among the game files the server reads.
pub const MOTIONS_GAME_FILE: &str = "EXPORT/MOTIONS.TOML";

#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MotionTimings {
    #[serde(default)]
    pub motion: Vec<MotionTiming>,
}

/// One motion file: its frame rate, length in frames and the frames it hits on.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MotionTiming {
    /// The ZMO path inside the client data, as FILE_MOTION.STB names it.
    pub file: String,
    pub fps: u32,
    pub frames: u32,
    #[serde(default)]
    pub hit_frames: Vec<u32>,
}

impl MotionTiming {
    /// Milliseconds of a frame, rounded down as the server's durations are.
    pub fn frame_ms(&self, frame: u32) -> u64 {
        frame as u64 * 1000 / self.fps.max(1) as u64
    }

    /// When `frame` starts, computed as the ZMO reader does so the server's times stay the
    /// same to the nanosecond.
    pub fn frame_time(&self, frame: u32) -> Duration {
        Duration::from_nanos(frame as u64 * 1_000_000_000 / self.fps.max(1) as u64)
    }

    pub fn duration(&self) -> Duration {
        self.frame_time(self.frames)
    }

    pub fn first_hit_time(&self) -> Option<Duration> {
        self.hit_frames.iter().min().map(|&f| self.frame_time(f))
    }
}

impl MotionTimings {
    pub fn parse(text: &str) -> Result<Self, anyhow::Error> {
        let timings: Self = toml::from_str(text)?;
        if let Some(bad) = timings.motion.iter().find(|m| m.fps == 0) {
            anyhow::bail!("{} has fps = 0", bad.file);
        }
        Ok(timings)
    }

    /// Read the file; a missing file means no timings.
    pub fn load(path: &Path) -> Result<Self, anyhow::Error> {
        match std::fs::read_to_string(path) {
            Ok(text) => Self::parse(&text).with_context(|| format!("reading {}", path.display())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(e) => Err(e).with_context(|| format!("reading {}", path.display())),
        }
    }

    /// The file as the export writes it: sorted by file, with the times in milliseconds as
    /// comments so a pull request shows what changed.
    pub fn to_toml(&self) -> String {
        let mut motions: Vec<&MotionTiming> = self.motion.iter().collect();
        motions.sort_by(|a, b| a.file.cmp(&b.file));
        let mut out = String::from(
            "# Motion timings for the server: length and hit frames of each player, monster and NPC\n\
             # motion file.\n\
             # Exported from the hit markers of the Godot animations; don't edit by hand. Change\n\
             # the markers in the Godot editor and run\n\
             #   godot --headless --path godot -- --export-data\n\
             # See export/README.md.\n",
        );
        for m in motions {
            out.push_str(&format!("\n[[motion]]\nfile = \"{}\"\nfps = {}\n", m.file, m.fps));
            out.push_str(&format!("frames = {}  # {} ms\n", m.frames, m.frame_ms(m.frames)));
            if !m.hit_frames.is_empty() {
                let frames: Vec<String> = m.hit_frames.iter().map(|f| f.to_string()).collect();
                let times: Vec<String> = m.hit_frames.iter().map(|&f| format!("{} ms", m.frame_ms(f))).collect();
                out.push_str(&format!("hit_frames = [{}]  # {}\n", frames.join(", "), times.join(", ")));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips() {
        let timings = MotionTimings {
            motion: vec![
                MotionTiming { file: "B.ZMO".into(), fps: 30, frames: 21, hit_frames: vec![] },
                MotionTiming { file: "A.ZMO".into(), fps: 30, frames: 37, hit_frames: vec![17] },
            ],
        };
        let text = timings.to_toml();
        assert!(text.contains("frames = 37  # 1233 ms\nhit_frames = [17]  # 566 ms\n"));
        let mut parsed = MotionTimings::parse(&text).unwrap();
        parsed.motion.sort_by(|a, b| b.file.cmp(&a.file));
        assert_eq!(parsed, timings);
    }

    #[test]
    fn rejects_unknown_keys_and_zero_fps() {
        assert!(MotionTimings::parse("[[motion]]\nfile = \"A\"\nfps = 30\nframes = 1\nhits = [1]\n").is_err());
        assert!(MotionTimings::parse("[[motion]]\nfile = \"A\"\nfps = 0\nframes = 1\n").is_err());
    }
}
