use reqwest;
use serde_json::{Value, json};
use std::error::Error;

pub async fn search_anime(allanime_api: &str, query: &str, mode: &str, agent: &str, allanime_refr: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let search_gql = r#"query($search: SearchInput, $limit: Int, $page: Int, $translationType: VaildTranslationTypeEnumType, $countryOrigin: VaildCountryOriginEnumType) {
        shows(search: $search, limit: $limit, page: $page, translationType: $translationType, countryOrigin: $countryOrigin) {
            edges {
                _id
                name
                availableEpisodes
                __typename
            }
        }
    }"#;

    let client = reqwest::Client::new();
    let resp = client.post(allanime_api)
        .header("User-Agent", agent)
        .header("Referer", allanime_refr)
        .json(&json!({
            "query": search_gql,
            "variables": {
                "search": {
                    "allowAdult": false,
                    "allowUnknown": false,
                    "query": query
                },
                "limit": 40,
                "page": 1,
                "translationType": mode,
                "countryOrigin": "ALL"
            }
        }))
        .send().await?
        .text().await?;

    let json_resp: Value = serde_json::from_str(&resp)?;
    let mut results = Vec::new();

    if let Some(edges) = json_resp["data"]["shows"]["edges"].as_array() {
        for edge in edges {
            let id = edge["_id"].as_str().unwrap_or_default().to_string();
            let name = edge["name"].as_str().unwrap_or_default().to_string();
            let episodes = edge["availableEpisodes"].to_string();
            results.push((id, format!("{} ({} episodes)", name, episodes)));
        }
    }

    Ok(results)
}

pub async fn episodes_list(allanime_api: &str, show_id: &str, mode: &str, agent: &str, allanime_refr: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let episodes_list_gql = r#"query ($showId: String!) {
        show(_id: $showId) {
            _id
            availableEpisodesDetail
        }
    }"#;

    let client = reqwest::Client::new();
    let resp = client.post(allanime_api)
        .header("User-Agent", agent)
        .header("Referer", allanime_refr)
        .json(&json!({
            "query": episodes_list_gql,
            "variables": {
                "showId": show_id
            }
        }))
        .send().await?
        .text().await?;
    println!("Response Text: {}", resp);
    let json_resp: Value = serde_json::from_str(&resp)?;
    println!("JSON Response: {:?}", json_resp);
    let mut episodes = Vec::new();

    if let Some(episodes_detail) = json_resp["data"]["show"]["availableEpisodesDetail"].get(mode) {
        if let Some(episodes_array) = episodes_detail.as_array() {
            for episode in episodes_array {
                if let Some(ep_number) = episode.as_str() {
                    episodes.push(ep_number.to_string());
                }
            }
        }
    }

    Ok(episodes)
}