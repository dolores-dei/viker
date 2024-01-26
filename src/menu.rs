use regex::Regex;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn list_anime(animes: Vec<(String, String)>) -> Option<String> {
    //takes vec ( anime id , show name ) returns just the anime id
    let mut fzf = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start fzf");


    let stdin = fzf.stdin.as_mut().expect("Failed to open stdin");
    for anime in animes.iter() {
        writeln!(stdin, "{} - {}", anime.0, anime.1).expect("Failed to write to stdin");
    }


    let output = fzf.wait_with_output().expect("Failed to read stdout");
    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);

        // Adjust the regular expression to match the ID before the first hyphen
        let re = Regex::new(r"^(\w+) -").unwrap();

        if let Some(caps) = re.captures(&output_str) {
            if let Some(id_match) = caps.get(1) {
                return Some(id_match.as_str().to_string());
            }
        }
        None
    } else {
        None
    }
}

pub fn list_episodes(episodes: Vec<String>) -> Option<String>{

    let mut fzf = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start fzf");

    let stdin = fzf.stdin.as_mut().expect("Failed to open stdin");

    for episode in episodes {
        writeln!(stdin, "{}", episode).expect("Failed to write to stdin");
    }

    let output = fzf.wait_with_output().expect("Failed to read stdout");
    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("FZF OUTPUT: {}", output_str);
        return Some(output_str.to_string().trim().to_string());
    }
    None
}