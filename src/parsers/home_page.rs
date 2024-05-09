use crate::types::errors::DefaultError;

use crate::utils::env;
use crate::config::default_env;
use crate::types::parsers::home_page::*;

use scraper::{Html, Selector};

pub async fn get() -> Result<warp::reply::WithStatus<warp::reply::Json>, DefaultError> {

    let response: HomePage = HomePage { 
        spotlight: vec![],
    };
    let home_page_url = env::get("DOMAIN_NAME", Some(default_env::SRC_BASE_URL))?;

    let http_client = reqwest::Client::new();
    let home_page_html = http_client.get(format!("{}{}", home_page_url, default_env::SRC_HOME_URL))
        .header("User-Agent", default_env::USER_AGENT_HEADER)
        .header("Accept", default_env::ACCEPT_HEADER)
        .send().await;
    if let Err(_) = home_page_html { return Err(DefaultError { message: "Failed to fetch home page html".to_string() }); }
    let home_page_html = home_page_html.unwrap().text().await.unwrap();

    let home_page = Html::parse_document(home_page_html.as_str());

    let spotlight_selector = Selector::parse("#slider .swiper-wrapper .swiper-slide").unwrap();

    for spotlight_elem in home_page.select(&spotlight_selector) {
        
    }

    Ok(warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::OK))
        
}