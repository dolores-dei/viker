use reqwest;
use serde_json::{json, Value};
use std::error::Error;
use std::process::Command;

#[derive(Debug)]
pub struct ProvideUrl {
    provider: Provider,
    pub(crate) url: String,
}

impl ProvideUrl {
    fn new(mut provider: String, mut url: String) -> Option<Self> {
        if url.contains("--") {
            let temp = url.replace("--", "");
            url = temp.trim().to_string();
        } else if url.contains("http") || url.contains("://") || url.contains("//") {
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
    if resp.len() < 30 { return Err("non valid response: too small".into()); }
    let mut command = format!("printf \"%s\" {} | head -1 | cut -d':' -f2 | sed 's/../&\\n/g' | sed 's/^01$/9/g;s/^08$/0/g;s/^05$/=/g;s/^0a$/2/g;s/^0b$/3/g;s/^0c$/4/g;s/^07$/?/g;s/^00$/8/g;s/^5c$/d/g;s/^0f$/7/g;s/^5e$/f/g;s/^17$/\\//g;s/^54$/l/g;s/^09$/1/g;s/^48$/p/g;s/^4f$/w/g;s/^0e$/6/g;s/^5b$/c/g;s/^5d$/e/g;s/^0d$/5/g;s/^53$/k/g;s/^1e$/\\&/g;s/^5a$/b/g;s/^59$/a/g;s/^4a$/r/g;s/^4c$/t/g;s/^4e$/v/g;s/^57$/o/g;s/^51$/i/g;' | tr -d '\\n'", resp);
    command.replace("clock", "clock.json");

    let output = Command::new("sh").arg("-c").arg(&command).output()?;

    if !output.status.success() {
        return Err("Command execution failed".into());
    }

    let output = String::from_utf8(output.stdout)?
        .trim()
        .to_string()
        .replace("clock", "clock.json");
    Ok(output)
}

#[test]
fn test_provider_init_success() {
    let resp = "175948514e4c4f57175b54575b5307515c050f5c0a0c0f0b0f0c0e590a0c0b5b0a0c0a010a010f080e5e0e0a0f0d0f0a0f0c0e0b0e0f0e5a0e5e0e000e090a000e5e0e010a010e590e010e0f0e0a0a000f0e0e5d0f0e0b010e5e0e0a0b5a0c5a0d0a0c5a0f5b0c010d0a0c0f0b0d0a080f0a0e5e0f0a0e590e0b0b5a0c0c0e010e5c0f0b0a5c0e000e010a5c0c5d0e0b0f0c0e010a5c0c0f0e0d0e0f0e0a0e0b0e5a0e5e0e0f0a5c0b0a0f0a0e5d0a5c0d0d0e0b0e0f0f0d0e010e000a080f0a0f5e0f0e0e0b0f0d0f0b0e0c0b5a0d0d0d0b0c0c0a080f0d0f0b0e0c0b5a0a080e0d0e010f080e0b0f0c0b5a0d5e0b0c0b5e0b0c0d5b0d5d0c5e0f080d5e0e5a0b5e0f0c0e0a0d0d0b0f0f0b0e0c0f5e0b0f0e010d5b0d5d0c5b0f080c590d090c080e5b0d5e0d090d0c0e590e0c0d090e590e5d0c590d0a0d0c0b0e0e0f0c0d0b0f0f5b0d5b0d090c080f5b0e0c0b0c0b0a0f0b0e0d0c090b0b0e000a0c0a590a0c0f0d0f0a0f0c0e0b0e0f0e5a0e0b0f0c0c5e0e0a0a0c0b5b0a0c0f080e5e0e0a0e0b0e010f0d0f0a0f0c0e0b0e0f0e5a0e5e0e010a0c0a590a0c0e0a0e0f0f0a0e0b0a0c0b5b0a0c0b0c0b0e0b0c0b0a0a5a0b0e0b0f0a5a0b0c0b080d0a0b0f0b0d0b5b0b0e0b0d0b5b0b0e0b0e0a000b0e0b0e0b0e0d5b0a0c0f5a1e4a5d5e5d4a5d4a05";
    let result = provider_init(resp).unwrap();
    println!("{:?}", result);
    assert!(result.contains("clock.json"));
}

#[test]
fn test_provider_init_error() {
    let resp = "invalid response";
    let result = provider_init(resp);

    assert!(result.is_err());
}
