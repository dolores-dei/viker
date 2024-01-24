use clap::Parser;

#[derive(Parser,Debug)]
struct Args {
    #[arg(short, long)]
    anime_name: Option<String>,
    #[arg(short, long, default_value = "vlc")]
    video_player: Option<String>

}

fn main() {
    let cli = Args::parse();

    if let Some(name) = cli.anime_name {
        println!("{}", name);
    }
    if let Some(video) = cli.video_player {
        println!("{}", video);
    }

}
