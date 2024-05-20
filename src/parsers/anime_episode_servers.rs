use crate::types::errors::DefaultError;
use crate::types::parsers::anime_episode_servers::*;

use crate::utils::env;
use crate::config::default_env;
use scraper::{Html, Selector};

pub async fn get(anime_id: String) -> Result<warp::reply::WithStatus<warp::reply::Json>, DefaultError> {

    let anime_id_end = anime_id.split("?ep=").last().unwrap();

    let mut response = AnimeEpisodeServers { 
        sub: Vec::new(),
        dub: Vec::new(),
        raw: Vec::new(),
    };
    let domain_url = env::get("DOMAIN_NAME", Some(default_env::SRC_BASE_URL))?;
    let episode_servers_ajax_url = format!("{}{}/v2/episode/servers?episodeId={}", domain_url, default_env::SRC_AJAX_URL, anime_id_end);

    let http_client = reqwest::Client::new();
    let episode_servers_html = http_client.get(episode_servers_ajax_url)
        .header("User-Agent", default_env::USER_AGENT_HEADER)
        .header("Accept", default_env::ACCEPT_HEADER)
        .send().await;
    if let Err(_) = episode_servers_html { return Err(DefaultError { message: "Failed to fetch home page html".to_string() }); }
    let episode_servers_html = episode_servers_html.unwrap().text().await.unwrap();
    let episode_servers_html: Result<EpisodeServersHtml, serde_json::Error> = serde_json::from_str(episode_servers_html.as_str());
    if let Err(err) = episode_servers_html { return Err(DefaultError { message: err.to_string() }); }
    let episode_servers_html = episode_servers_html.unwrap();

    let episode_servers_elem = Html::parse_document(episode_servers_html.html.as_str());

    let server_types = vec!["sub","dub","raw"];

    for server_type in server_types {

        for episode_server_elem in episode_servers_elem.select(&Selector::parse(format!(".ps_-block.ps_-block-sub.servers-{} .ps__-list .server-item", server_type).as_str()).unwrap()) {
            let mut server = AnimeEpisodeServer {
                name: String::new(),
                id: 0
            };
    
            server.name = episode_server_elem.select(&Selector::parse("a").unwrap()).next().expect("Server name not found")
                .text().collect::<String>().trim().to_lowercase().to_string();
            
            server.id = episode_server_elem.attr("data-server-id").unwrap_or("0").trim().parse().expect("Failed to convert server id to integer");
            
            match server_type {
                "sub" => response.sub.push(server),
                "dub" => response.dub.push(server),
                _ => response.raw.push(server)
            }
        }

    }    

    Ok(warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::OK))
        
}