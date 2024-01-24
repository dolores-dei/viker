mod web;

use clap::Parser;
use std::env::consts::{OS, ARCH};

use select::predicate::{Name, Class, Predicate};

#[derive(Parser,Debug)]
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

#[tokio::main]
async fn main() {
    let cli = Args::parse();

    let mut url = String::from("https://ww4.gogoanime2.org/search/");

    if let Some(name) = cli.anime_name {
        println!("{}", name);
        url.push_str(&name);
    }
    println!("{}", cli.video_player);

    let res = web::get_anime_titles(&url).await;
    println!("anime titles: {:#?}", res.unwrap());

}
