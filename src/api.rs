use reqwest;
use serde_json::{json, Value};
use std::error::Error;
use std::process::Command;

#[derive(Debug)]
pub struct ProvideUrl {
    provider: Provider,
    url: String,
}

impl ProvideUrl {
    fn new(mut provider: String, mut url: String) -> Option<Self> {
        if url.contains("--") {
            let temp = url.replace("--", "");
            url = temp.trim().to_string();
        } else if url.contains("http") || url.contains("://") {
            return None;
        }
        let provider = match provider.as_str() {
            "Sak" => Provider::Dropbox,
            "Luf-mp4" => Provider::Gogoanime,
            "S-mp4" => Provider::Dropbox,
            _ => Provider::Other,
        };
        Some(Self { provider, url })
    }
}

#[derive(Debug)]
enum Provider {
    Wixmp,
    Dropbox,
    Wetransfer,
    Sharepoint,
    Gogoanime,
    Other,
}

pub async fn search_anime(
    allanime_api: &str,
    query: &str,
    mode: &str,
    agent: &str,
    allanime_refr: &str,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
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
    let resp = client
        .post(allanime_api)
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
        .send()
        .await?
        .text()
        .await?;

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

pub async fn episodes_list(
    allanime_api: &str,
    show_id: &str,
    mode: &str,
    agent: &str,
    allanime_refr: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let episodes_list_gql = r#"query ($showId: String!) {
        show(_id: $showId) {
            _id
            availableEpisodesDetail
        }
    }"#;

    let client = reqwest::Client::new();
    let resp = client
        .post(allanime_api)
        .header("User-Agent", agent)
        .header("Referer", allanime_refr)
        .json(&json!({
            "query": episodes_list_gql,
            "variables": {
                "showId": show_id
            }
        }))
        .send()
        .await?
        .text()
        .await?;
    println!("Response Text: {}", resp);
    let json_resp: Value = serde_json::from_str(&resp)?;
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

pub async fn get_episode_url(
    allanime_api: &str,
    show_id: &str,
    episode_string: &str,
    mode: &str,
    agent: &str,
    allanime_refr: &str,
) -> Result<Vec<ProvideUrl>, Box<dyn Error>> {
    let episode_embed_gql = r#"query ($showId: String!, $translationType: VaildTranslationTypeEnumType!, $episodeString: String!) {
        episode(showId: $showId, translationType: $translationType, episodeString: $episodeString) {
            episodeString
            sourceUrls
        }
    }"#;

    let client = reqwest::Client::new();
    let resp = client
        .post(allanime_api)
        .header("User-Agent", agent)
        .header("Referer", allanime_refr)
        .json(&json!({
            "query": episode_embed_gql,
            "variables": {
                "showId": show_id,
                "translationType": mode,
                "episodeString": episode_string
            }
        }))
        .send()
        .await?
        .text()
        .await?;

    let json_resp: Value = serde_json::from_str(&resp)?;
    //println!(" json resp: {}", json_resp); //used for debug
    let mut provider_urls = Vec::new();

    if let Some(sources) = json_resp["data"]["episode"]["sourceUrls"].as_array() {
        for source in sources {
            if let (Some(provider_name), Some(url)) =
                (source["sourceName"].as_str(), source["sourceUrl"].as_str())
            {
                if let Some(provider_url) =
                    ProvideUrl::new(provider_name.to_string(), url.to_string())
                {
                    provider_urls.push(provider_url);
                }
            }
        }
    }

    Ok(provider_urls)
}

pub fn provider_init(resp: &str) -> Result<String, Box<dyn Error>> {
    let mut command = format!("printf \"%s\" {} | head -1 | cut -d':' -f2 | sed 's/../&\\n/g' | sed 's/^01$/9/g;s/^08$/0/g;s/^05$/=/g;s/^0a$/2/g;s/^0b$/3/g;s/^0c$/4/g;s/^07$/?/g;s/^00$/8/g;s/^5c$/d/g;s/^0f$/7/g;s/^5e$/f/g;s/^17$/\\//g;s/^54$/l/g;s/^09$/1/g;s/^48$/p/g;s/^4f$/w/g;s/^0e$/6/g;s/^5b$/c/g;s/^5d$/e/g;s/^0d$/5/g;s/^53$/k/g;s/^1e$/\\&/g;s/^5a$/b/g;s/^59$/a/g;s/^4a$/r/g;s/^4c$/t/g;s/^4e$/v/g;s/^57$/o/g;s/^51$/i/g;' | tr -d '\\n'", resp);
    command.replace("clock", "clock.json");

    let output = Command::new("sh").arg("-c").arg(&command).output()?;


    if !output.status.success() {
        return Err("Command execution failed".into());
    }

    let output= String::from_utf8(output.stdout)?.trim().to_string().replace("clock", "clock.json");
    Ok(output)
}
