use clap::Parser;
use std::env::consts::{OS, ARCH};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    anime_name: Option<String>,

    #[arg(short, long, default_value_t = default_video_player())]
    video_player: String,
}

fn default_video_player() -> String {
    match (OS, ARCH) {
        ("macos", "aarch64") => "iina".to_string(), // Apple Silicon
        ("linux", "x86_64") => "vlc".to_string(),  // Linux AMD64
        _ => "vlc".to_string(),                    // Default for other OS/Arch combinations
    }
}

fn main() {
    let cli = Args::parse();

    if let Some(name) = cli.anime_name {
        println!("{}", name);
    }
    println!("{}", cli.video_player);

}