use std::process::{Command, Stdio};
use std::io::Write;

pub fn list_anime(animes: Vec<String>) -> Option<String> {
    // Create a command to run fzf
    let mut fzf = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start fzf");
    {
        // Write anime list to the stdin of fzf
        let stdin = fzf.stdin.as_mut().expect("Failed to open stdin");
        for anime in animes {
            writeln!(stdin, "{}", anime).expect("Failed to write to stdin");
        }
    }

    let output = fzf.wait_with_output().expect("Failed to read stdout");
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}
