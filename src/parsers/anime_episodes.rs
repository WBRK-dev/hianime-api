use crate::types::errors::DefaultError;
use crate::types::parsers::anime_episodes::*;

use crate::utils::env;
use crate::utils::anime::{scrape_anime_title, scrape_anime_jtitle};
use crate::config::default_env;
use scraper::{CaseSensitivity, Html, Selector};

pub async fn get(anime_id: String) -> Result<warp::reply::WithStatus<warp::reply::Json>, DefaultError> {

    let anime_id_end = anime_id.split("-").last().unwrap();

    let mut response = AnimeEpisodes { 
        episodes: Vec::new()
    };
    let domain_url = env::get("DOMAIN_NAME", Some(default_env::SRC_BASE_URL))?;
    let episode_ajax_url = format!("{}{}/v2/episode/list/{}", domain_url, default_env::SRC_AJAX_URL, anime_id_end);

    let http_client = reqwest::Client::new();
    let episodes_html = http_client.get(episode_ajax_url)
        .header("User-Agent", default_env::USER_AGENT_HEADER)
        .header("Accept", default_env::ACCEPT_HEADER)
        .send().await;
    if let Err(_) = episodes_html { return Err(DefaultError { message: "Failed to fetch home page html".to_string() }); }
    let episodes_html = episodes_html.unwrap().text().await.unwrap();
    let episodes_html: Result<EpisodeHtml, serde_json::Error> = serde_json::from_str(episodes_html.as_str());
    if let Err(err) = episodes_html { return Err(DefaultError { message: err.to_string() }); }
    let episodes_html = episodes_html.unwrap();

    let episodes_elem = Html::parse_document(episodes_html.html.as_str());
    
    for episode_elem in episodes_elem.select(&Selector::parse(".detail-infor-content .ss-list a").unwrap()) {
        let mut anime_episode = AnimeEpisode {
            title: String::new(),
            jtitle: String::new(),
            episode_id: String::new(),
            number: 0,
            is_filler: false,
        };

        anime_episode.title = scrape_anime_title(episode_elem.select(&Selector::parse(".ep-name.e-dynamic-name").unwrap()));
        anime_episode.jtitle = scrape_anime_jtitle(episode_elem.select(&Selector::parse(".ep-name.e-dynamic-name").unwrap()));

        anime_episode.episode_id = episode_elem.attr("href").unwrap_or("").split("/").last().unwrap_or("").to_string();

        anime_episode.number = episode_elem.attr("data-number").unwrap_or("0").parse().expect("Failed to convert string to int");

        anime_episode.is_filler = episode_elem.value().has_class("ssl-item-filler", CaseSensitivity::AsciiCaseInsensitive);

        response.episodes.push(anime_episode);
    }

    Ok(warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::OK))
        
}