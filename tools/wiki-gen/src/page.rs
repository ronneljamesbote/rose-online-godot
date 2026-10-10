//! A wiki page being written: a YAML data block, built in order, and a Markdown body.

use std::{collections::BTreeMap, fmt::Write as _, fs, path::Path};

use serde_yaml::{Mapping, Value};

pub struct Page {
    pub path: String,
    pub data: Mapping,
    pub body: String,
}

impl Page {
    pub fn new(path: impl Into<String>, kind: &str) -> Self {
        let mut data = Mapping::new();
        data.insert("kind".into(), kind.into());
        Self { path: path.into(), data, body: String::new() }
    }

    pub fn set(&mut self, key: &str, value: impl Into<Value>) -> &mut Self {
        self.data.insert(key.into(), value.into());
        self
    }

    /// Set only when the value says something: not empty, not zero, not false.
    pub fn set_some(&mut self, key: &str, value: impl Into<Value>) -> &mut Self {
        let value = value.into();
        let empty = match &value {
            Value::Null => true,
            Value::Bool(b) => !b,
            Value::Number(n) => n.as_f64() == Some(0.0),
            Value::String(s) => s.trim().is_empty(),
            Value::Sequence(s) => s.is_empty(),
            Value::Mapping(m) => m.is_empty(),
            Value::Tagged(_) => false,
        };
        if !empty {
            self.data.insert(key.into(), value);
        }
        self
    }

    pub fn line(&mut self, text: impl AsRef<str>) -> &mut Self {
        self.body.push_str(text.as_ref());
        self.body.push('\n');
        self
    }

    pub fn write(&self, root: &Path) -> anyhow::Result<()> {
        let file = if self.path.is_empty() {
            root.join("index.md")
        } else if self.data.get("kind").and_then(|k| k.as_str()) == Some("list") {
            root.join(&self.path).join("index.md")
        } else {
            root.join(format!("{}.md", self.path))
        };
        if let Some(parent) = file.parent() {
            fs::create_dir_all(parent)?;
        }
        let yaml = serde_yaml::to_string(&self.data)?;
        let mut text = String::with_capacity(yaml.len() + self.body.len() + 16);
        let _ = write!(text, "---\n{yaml}---\n");
        text.push_str(&self.body);
        fs::write(file, text)?;
        Ok(())
    }
}

/// A row of a table in the data block.
pub fn row<const N: usize>(pairs: [(&str, Value); N]) -> Value {
    let mut m = Mapping::new();
    for (k, v) in pairs {
        if !v.is_null() {
            m.insert(k.into(), v);
        }
    }
    Value::Mapping(m)
}

pub fn map(pairs: Vec<(&str, Value)>) -> Value {
    let mut m = Mapping::new();
    for (k, v) in pairs {
        if !v.is_null() {
            m.insert(k.into(), v);
        }
    }
    Value::Mapping(m)
}

pub fn seq(values: impl IntoIterator<Item = Value>) -> Value {
    Value::Sequence(values.into_iter().collect())
}

pub fn strs(values: impl IntoIterator<Item = String>) -> Value {
    Value::Sequence(values.into_iter().map(Value::String).collect())
}

/// Metres with one decimal, from the game's centimetres.
pub fn metres(cm: f32) -> Value {
    round1(cm / 100.0)
}

pub fn round1(v: f32) -> Value {
    let r = (v as f64 * 10.0).round() / 10.0;
    if r.fract() == 0.0 {
        Value::from(r as i64)
    } else {
        Value::from(r as f64)
    }
}

/// "Short Sword" -> "short-sword".
pub fn slug(name: &str) -> String {
    let mut out = String::new();
    let mut dash = false;
    for ch in name.chars().flat_map(|c| c.to_lowercase()) {
        if ch.is_ascii_alphanumeric() {
            if dash && !out.is_empty() {
                out.push('-');
            }
            dash = false;
            out.push(ch);
        } else {
            dash = true;
        }
        if out.len() >= 48 {
            break;
        }
    }
    if out.is_empty() {
        "unnamed".into()
    } else {
        out
    }
}

/// A [[link]] for the data block.
pub fn link(path: &str, label: &str) -> String {
    format!("[[{path}|{}]]", clean_label(label))
}

/// A [[link]] for a Markdown table in the body, where | ends a cell.
pub fn table_link(path: &str, label: &str) -> String {
    format!("[[{path}\\|{}]]", clean_label(label))
}

fn clean_label(label: &str) -> String {
    let l = label.replace(['|', '[', ']'], " ");
    let l = l.split_whitespace().collect::<Vec<_>>().join(" ");
    if l.is_empty() {
        "(no name)".into()
    } else {
        l
    }
}

/// ROSE's text tags ({br}, {b}, {fc=N}, <NAME>) to Markdown.
pub fn rose_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find(['<', '{']) {
        let (before, tag) = rest.split_at(start);
        out += before;
        let close = if tag.starts_with('<') { '>' } else { '}' };
        let Some(end) = tag.find(close) else {
            rest = tag;
            break;
        };
        let (tag, after) = tag.split_at(end + 1);
        match tag.to_lowercase().as_str() {
            "<name>" => out += "(your name)",
            "<level>" => out += "(your level)",
            "{br}" => out += "\n",
            "{b}" | "{/b}" => out += "**",
            t if t.starts_with('<') => out += t,
            _ => {}
        }
        rest = after;
    }
    out += rest;
    out.replace('\r', "").trim().to_string()
}

/// Text on one line, for a list item or table cell.
pub fn one_line(text: &str) -> String {
    rose_text(text).split('\n').map(str::trim).filter(|l| !l.is_empty()).collect::<Vec<_>>().join(" ")
}

/// Natural sort key: "10-2" after "9-10".
pub fn natural_key(s: &str) -> Vec<(String, u64)> {
    let mut out = Vec::new();
    let mut text = String::new();
    let mut num: Option<u64> = None;
    for ch in s.chars() {
        if let Some(d) = ch.to_digit(10) {
            num = Some(num.unwrap_or(0).saturating_mul(10).saturating_add(d as u64));
        } else {
            if let Some(n) = num.take() {
                out.push((std::mem::take(&mut text), n));
            }
            text.push(ch);
        }
    }
    out.push((text, num.unwrap_or(0)));
    out
}

/// All pages, by path, so later steps can add to earlier ones before writing.
#[derive(Default)]
pub struct Pages(pub BTreeMap<String, Page>);

impl Pages {
    pub fn add(&mut self, page: Page) {
        self.0.insert(page.path.clone(), page);
    }
}
