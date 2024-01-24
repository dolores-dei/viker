use reqwest::Client;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

pub(crate) async fn get_anime_titles(url: &str) -> Result<(Vec<String>), reqwest::Error> {
    let mut anime_names: Vec<String> = vec![];

    let response = Client::new().get(url).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;

        let document = Document::from(body.as_str());

        for node in document.find(Class("last_episodes").descendant(Class("items"))) {
            for li in node.find(Name("a")){
                let mut anime_title = li.text();
                anime_title = anime_title.trim().parse().unwrap();
                if !anime_title.is_empty(){
                    anime_names.push(anime_title);
                }
            }
        }

        Ok(anime_names)
    } else {
        println!("Failed to retrieve the page. Status code: {:?}", response.status());
        Err(response.error_for_status().unwrap_err())
    }
}