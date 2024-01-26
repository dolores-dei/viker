use std::env::consts::{ARCH, OS};
use clap::Parser;
use crate::api::{get_episode_url, provider_init};

mod menu;
mod api;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, value_name = "ANIME_NAME")]
    anime_name: Option<String>,

    #[arg(short, long, default_value_t = default_video_player(), value_name = "VIDEO_PLAYER")]
    video_player: String,
}

fn default_video_player() -> String {
    match (OS, ARCH) {
        ("macos", "aarch64") => "iina".to_string(), // Apple Silicon
        ("linux", "x86_64") => "vlc".to_string(),   // Linux AMD64
        _ => "vlc".to_string(),                     // Default for other OS/Arch combinations
    }
}


#[tokio::main]
async fn main() {

    let args = Args::parse();
    let anime_name = args
        .anime_name
        .unwrap_or("failed anime name entry".to_string());

    let allanime_api = "https://api.allanime.day/api";
    let query = anime_name.as_str(); // later Replace with user's search query
    let mut anime_id: Option<String> = None; // onl for testing
    let mode = "sub"; // or "dub", user preference
    let agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/121.0";
    let allanime_refr = "https://allanime.to";


    match api::search_anime(allanime_api, query, mode, agent, allanime_refr).await {
        Ok(anime_list) => {
            anime_id = menu::list_anime(anime_list);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    match api::episodes_list(allanime_api, anime_id.clone().unwrap().as_str(), mode, agent, allanime_refr).await {
        Ok(episodes) => {
            let choice = menu::list_episodes(episodes);
            println!("{:#?}", choice.clone().unwrap());
            let links = get_episode_url(allanime_api,&anime_id.unwrap(),&choice.unwrap(),mode,agent,allanime_refr).await;
            println!("{:?}",links.unwrap());
        }
        Err(e) => eprintln!("Error: {}", e),
    };
    let  ulet = provider_init("175948514e4c4f57175b54575b5307515c050f5c0a0c0f0b0f0c0e590a0c0b5b0a0c0a010a010f080e5e0e0a0f0d0f0a0f0c0e0b0e0f0e5a0e5e0e000e090a000e5e0e010a010e590e010e0f0e0a0a000f0e0e5d0f0e0b010e5e0e0a0b5a0c5a0d0a0c5a0f5b0c010d0a0c0f0b0d0a080f0a0e5e0f0a0e590e0b0b5a0c0c0e010e5c0f0b0a5c0e000e010a5c0c5d0e0b0f0c0e010a5c0c0f0e0d0e0f0e0a0e0b0e5a0e5e0e0f0a5c0b0a0f0a0e5d0a5c0d0d0e0b0e0f0f0d0e010e000a080f0a0f5e0f0e0e0b0f0d0f0b0e0c0b5a0d0d0d0b0c0c0a080f0d0f0b0e0c0b5a0a080e0d0e010f080e0b0f0c0b5a0d5e0b0c0b5e0b0c0d5b0d5d0c5e0f080d5e0e5a0b5e0f0c0e0a0d0d0b0f0f0b0e0c0f5e0b0f0e010d5b0d5d0c5b0f080c590d090c080e5b0d5e0d090d0c0e590e0c0d090e590e5d0c590d0a0d0c0b0e0e0f0c0d0b0f0f5b0d5b0d090c080f5b0e0c0b0c0b0a0f0b0e0d0c090b0b0e000a0c0a590a0c0f0d0f0a0f0c0e0b0e0f0e5a0e0b0f0c0c5e0e0a0a0c0b5b0a0c0f080e5e0e0a0e0b0e010f0d0f0a0f0c0e0b0e0f0e5a0e5e0e010a0c0a590a0c0e0a0e0f0f0a0e0b0a0c0b5b0a0c0b0c0b0e0b0c0b0a0a5a0b0e0b0f0a5a0b0c0b080d0a0b0f0b0d0b5b0b0e0b0d0b5b0b0e0b0e0a000b0e0b0e0b0e0d5b0a0c0f5a1e4a5d5e5d4a5d4a05");
    println!("{:?}", ulet);
}

