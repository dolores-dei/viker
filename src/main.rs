use std::env::consts::{ARCH, OS};
use clap::Parser;
mod menu;
mod web;

#[derive(Parser,Debug)]
struct Args {
    #[clap(short, long, value_name = "ANIME_NAME")]
    anime_name: Option<String>,

    #[arg(short, long, default_value_t = default_video_player(), value_name = "VIDEO_PLAYER")]
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

    let args = Args::parse();
    let anime_name = args.anime_name.unwrap_or("failed anime name entry".to_string());

    let allanime_api = "https://api.allanime.day/api";
    let query = anime_name.as_str(); // later Replace with user's search query
    let anime_id = "Yr7ha4n76ofd7BeSX"; // onl for testing
    let mode = "sub"; // or "dub", user preference
    let agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/121.0";
    let allanime_refr = "https://allanime.to";

    let mut anime_result: Option<String> = Some("error".to_string());

    match web::search_anime(allanime_api, query, mode, agent, allanime_refr).await {
        Ok(anime_list) => {
            anime_result = menu::list_anime(anime_list);
            println!("SEARCH_ANIME --- {:?}", anime_result);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    match web::episodes_list(allanime_api, anime_result.unwrap().as_str(), mode, agent, allanime_refr).await {
        Ok(episodes) => {
            let choice = menu::list_episodes(episodes);
            println!("{:#?}", choice);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

}