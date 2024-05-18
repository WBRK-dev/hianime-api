use crate::types::anime::AnimeEpisodes;
use crate::types::parsers::home_page::Top10AnimeWrapper;
use crate::types::errors::DefaultError;

use crate::utils::env;
use crate::config::default_env;
use crate::types::parsers::home_page::*;
use crate::utils::anime::*;

use scraper::{Html, Selector};

pub async fn get() -> Result<warp::reply::WithStatus<warp::reply::Json>, DefaultError> {

    let mut response: HomePage = HomePage { 
        spotlight: Vec::new(),
        trending: Vec::new(),
        latest_episodes: Vec::new(),
        top_upcoming: Vec::new(),
        top10: Top10AnimeWrapper { day: Vec::new(), week: Vec::new(), month: Vec::new() },
        top_airing: Vec::new(),
        genres: Vec::new()
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

    for spotlight_elem in home_page.select(&Selector::parse("#slider .swiper-wrapper .swiper-slide").unwrap()) {
        let mut spotlight_item = SpotlightItem { 
            rank: 0,
            id: String::new(),
            title: String::new(),
            jtitle: String::new(),
            description: String::new(),
            poster: String::new(),
            details: Vec::new(),
            episodes: AnimeEpisodes {
                sub: 0,
                dub: 0,
            }
        };

        // Get spotlight rank
        spotlight_item.rank = spotlight_elem.select(&Selector::parse(".deslide-item-content .desi-sub-text").unwrap()).next().expect("Failed to find rank")
            .text().map(|rank| rank.trim()).collect::<String>().split(" ").next().expect("Failed to find rank").split("#").last().unwrap()
            .parse::<u8>().expect("Failed to convert rank string to int");

        spotlight_item.id = scrape_anime_id(spotlight_elem.select(&Selector::parse(".deslide-item-content .desi-buttons a").unwrap()));
        spotlight_item.title = scrape_anime_title(spotlight_elem.select(&Selector::parse(".deslide-item-content .desi-head-title.dynamic-name").unwrap()));
        spotlight_item.jtitle = scrape_anime_jtitle(spotlight_elem.select(&Selector::parse(".deslide-item-content .desi-head-title.dynamic-name").unwrap()));
        spotlight_item.description = scrape_anime_description(spotlight_elem.select(&Selector::parse(".deslide-item-content .desi-description").unwrap()));
        spotlight_item.poster = scrape_anime_poster(spotlight_elem.select(&Selector::parse(".deslide-cover .deslide-cover-img .film-poster-img").unwrap()));

        // Get spotlight episodes
        spotlight_item.episodes.sub = spotlight_elem.select(&Selector::parse(".deslide-item-content .sc-detail .scd-item .tick-item.tick-sub").unwrap())
            .next().expect("Failed to get subbed episode count").text().collect::<String>().trim().parse::<u32>().expect("Failed to get subbed episode count");

        if let Some(dub_elem) = spotlight_elem.select(&Selector::parse(".deslide-item-content .sc-detail .scd-item .tick-item.tick-dub").unwrap()).next() {
            spotlight_item.episodes.dub = dub_elem.text().collect::<String>().trim().parse::<u32>().expect("Failed to get dubbed episode count");
        }

        // Get spotlight anime details
        let mut details: Vec<String> = spotlight_elem.select(&Selector::parse(".deslide-item-content .sc-detail .scd-item").unwrap())
            .map(|detail| detail.text().map(|detaill| detaill.trim()).collect()).collect();
        let _ = details.pop();
        spotlight_item.details = details;

        response.spotlight.push(spotlight_item);
    }

    for trending_elem in home_page.select(&Selector::parse("#trending-home .swiper-wrapper .swiper-slide").unwrap()) {
        let mut trending_item = TrendingItem { 
            rank: 0,
            id: String::new(),
            title: String::new(),
            jtitle: String::new(),
            poster: String::new(),
        };

        trending_item.rank = scrape_anime_rank(trending_elem.select(&Selector::parse(".item .number span").unwrap()));
        trending_item.id = scrape_anime_id(trending_elem.select(&Selector::parse(".item .film-poster").unwrap()));
        trending_item.title = scrape_anime_title(trending_elem.select(&Selector::parse(".item .number .film-title.dynamic-name").unwrap()));
        trending_item.jtitle = scrape_anime_jtitle(trending_elem.select(&Selector::parse(".item .number .film-title.dynamic-name").unwrap()));
        trending_item.poster = scrape_anime_poster(trending_elem.select(&Selector::parse(".item .film-poster .film-poster-img").unwrap()));

        response.trending.push(trending_item);
    }

    response.latest_episodes = scrape_basic_anime(home_page.select(&Selector::parse("#main-content .block_area_home:nth-of-type(1) .tab-content .film_list-wrap .flw-item").unwrap()));
    response.top_upcoming = scrape_basic_anime(home_page.select(&Selector::parse("#main-content .block_area_home:nth-of-type(3) .tab-content .film_list-wrap .flw-item").unwrap()));

    for top10_period_wrapper in home_page.select(&Selector::parse(r#"#main-sidebar .block_area-realtime [id^="top-viewed-"]"#).unwrap()) {
        let period = top10_period_wrapper.attr("id").unwrap().split("-").last().expect("Period not found in top10 wrapper").trim();

        if period == "day" {
            response.top10.day = scrape_top10_anime(top10_period_wrapper, "day");
        } else if period == "week" {
            response.top10.week = scrape_top10_anime(top10_period_wrapper, "week");
        } else if period == "month" {
            response.top10.month = scrape_top10_anime(top10_period_wrapper, "month");
        }
    }

    for top_airing_elem in home_page.select(&Selector::parse("#anime-featured .row div:nth-of-type(1) .anif-block-ul ul li").unwrap()) {
        let mut top_airing_item = TopAiringItem {
            id: String::new(),
            title: String::new(),
            jtitle: String::new(),
            poster: String::new(),
            details: Vec::new()
        };

        top_airing_item.id = scrape_anime_id(top_airing_elem.select(&Selector::parse(".film-detail .film-name .dynamic-name").unwrap()));
        top_airing_item.title = scrape_anime_title(top_airing_elem.select(&Selector::parse(".film-detail .film-name .dynamic-name").unwrap()));
        top_airing_item.jtitle = scrape_anime_jtitle(top_airing_elem.select(&Selector::parse(".film-detail .film-name .dynamic-name").unwrap()));
        top_airing_item.poster = scrape_anime_poster(top_airing_elem.select(&Selector::parse(".film-poster a .film-poster-img").unwrap()));
        
        top_airing_item.details = top_airing_elem.select(&Selector::parse(".fd-infor .fdi-item").unwrap())
        .map(|detail| detail.text().collect::<String>().trim().to_string()).collect();

        response.top_airing.push(top_airing_item);
    }

    response.genres = home_page.select(&Selector::parse("#main-sidebar .block_area.block_area_sidebar.block_area-genres .sb-genre-list li").unwrap())
        .map(|genre| genre.text().collect::<String>().trim().to_string()).collect();

    Ok(warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::OK))
        
}