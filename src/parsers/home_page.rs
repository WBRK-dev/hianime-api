use crate::types::anime::AnimeEpisodes;
use crate::types::parsers::home_page::Top10AnimeWrapper;
use crate::types::errors::DefaultError;

use crate::utils::env;
use crate::config::default_env;
use crate::types::parsers::home_page::*;
use crate::utils::anime::{scrape_basic_anime, scrape_top10_anime};

use scraper::{Html, Selector};

pub async fn get() -> Result<warp::reply::WithStatus<warp::reply::Json>, DefaultError> {

    let mut response: HomePage = HomePage { 
        spotlight: Vec::new(),
        trending: Vec::new(),
        latest_episodes: Vec::new(),
        top_upcoming: Vec::new(),
        top10: Top10AnimeWrapper { day: Vec::new(), week: Vec::new(), month: Vec::new() },
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

        // Get spotlight anime id
        spotlight_item.id = spotlight_elem.select(&Selector::parse(".deslide-item-content .desi-buttons a").unwrap()).last()
            .expect("Anime id not found").attr("href").expect("Anime id not found").split("").skip(2).collect::<String>();

        // Get spotlight anime title
        spotlight_item.title = spotlight_elem.select(&Selector::parse(".deslide-item-content .desi-head-title.dynamic-name").unwrap()).next()
            .expect("Title not found").text().collect::<String>().trim().to_string();

        // Get spotlight anime japanese title
        spotlight_item.jtitle = spotlight_elem.select(&Selector::parse(".deslide-item-content .desi-head-title.dynamic-name").unwrap()).next()
            .expect("Japanese title not found").attr("data-jname").expect("Japanese title not found").trim().to_string();

        // Get spotlight anime description
        spotlight_item.description = spotlight_elem.select(&Selector::parse(".deslide-item-content .desi-description").unwrap()).next()
            .expect("Description not found").text().collect::<String>().trim().to_string();

        // Get spotlight anime poster url
        spotlight_item.poster = spotlight_elem.select(&Selector::parse(".deslide-cover .deslide-cover-img .film-poster-img").unwrap()).next()
            .expect("Poster url not found").attr("data-src").expect("Poster url not found").trim().to_string();

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

        // Get trending anime rank
        trending_item.rank = trending_elem.select(&Selector::parse(".item .number span").unwrap()).next().expect("Failed to find rank")
            .text().collect::<String>().parse().unwrap();

        // Get trending anime id
        trending_item.id = trending_elem.select(&Selector::parse(".item .film-poster").unwrap()).next().expect("Failed to find id")
            .attr("href").expect("Failed to find id").split("").skip(2).collect();

        // Get trending anime title
        trending_item.title = trending_elem.select(&Selector::parse(".item .number .film-title.dynamic-name").unwrap()).next()
            .expect("Title not found").text().collect::<String>().trim().to_string();

        // Get trending anime japanese title
        trending_item.jtitle = trending_elem.select(&Selector::parse(".item .number .film-title.dynamic-name").unwrap()).next()
            .expect("Japanese title not found").attr("data-jname").expect("Japanese title not found").trim().to_string();

        // Get trending anime poster url
        trending_item.poster = trending_elem.select(&Selector::parse(".item .film-poster .film-poster-img").unwrap()).next()
            .expect("Poster url not found").attr("data-src").expect("Poster url not found").trim().to_string();

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

    Ok(warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::OK))
        
}