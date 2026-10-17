//! Builds the git commit the module was built from into it (ROSE_GIT_COMMIT), so players
//! can see which server version they are on (the server_info view).
//! ROSE_GIT_COMMIT set in the environment wins (the Docker build passes it when it has no
//! git); otherwise git is asked, then the files in ../.git are read; "unknown" if all fail.

use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=ROSE_GIT_COMMIT");
    let git = Path::new("../.git");
    // A new commit changes the branch's ref file, not HEAD.
    println!("cargo:rerun-if-changed=../.git/HEAD");
    println!("cargo:rerun-if-changed=../.git/packed-refs");
    if let Some(name) = std::fs::read_to_string(git.join("HEAD")).ok().and_then(|h| h.trim().strip_prefix("ref: ").map(str::to_string)) {
        println!("cargo:rerun-if-changed=../.git/{name}");
    }
    let commit = std::env::var("ROSE_GIT_COMMIT")
        .ok()
        .filter(|c| !c.trim().is_empty())
        .or_else(git_command)
        .or_else(|| read_git_dir(git))
        .unwrap_or_else(|| "unknown".into());
    let commit: String = commit.trim().chars().take(7).collect();
    println!("cargo:rustc-env=ROSE_GIT_COMMIT={commit}");
}

fn git_command() -> Option<String> {
    let out = Command::new("git").args(["rev-parse", "--short=7", "HEAD"]).output().ok()?;
    out.status.success().then(|| String::from_utf8_lossy(&out.stdout).trim().to_string()).filter(|c| !c.is_empty())
}

/// HEAD is either a commit or "ref: refs/heads/NAME", found as a file under refs/ or a
/// line in packed-refs.
fn read_git_dir(git: &Path) -> Option<String> {
    let head = std::fs::read_to_string(git.join("HEAD")).ok()?;
    let head = head.trim();
    let Some(name) = head.strip_prefix("ref: ") else { return Some(head.to_string()) };
    if let Ok(commit) = std::fs::read_to_string(git.join(name)) {
        return Some(commit.trim().to_string());
    }
    let packed = std::fs::read_to_string(git.join("packed-refs")).ok()?;
    packed
        .lines()
        .find_map(|line| line.strip_suffix(name).map(|commit| commit.trim().to_string()))
        .filter(|c| !c.is_empty())
}
