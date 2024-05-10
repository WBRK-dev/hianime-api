use scraper::{html::Select, ElementRef};
use scraper::Selector;

use crate::types::{anime::{Anime, AnimeDetails, AnimeEpisodes}, parsers::home_page::Top10AnimeItem};

pub fn scrape_basic_anime(elems: Select) -> Vec<Anime> {
    let mut animes: Vec<Anime> = Vec::new();

    for anime_elem in elems {
        let mut anime = Anime {
            id: String::new(),
            title: String::new(),
            jtitle: String::new(),
            poster: String::new(),
            details: AnimeDetails { duration: String::new(), r#type: String::new(), rating: String::new() },
            episodes: AnimeEpisodes { sub: 0, dub: 0 }
        };

        // Get anime id
        anime.id = anime_elem.select(&Selector::parse(".film-detail .film-name .dynamic-name").unwrap()).next().expect("Failed to find id")
            .attr("href").expect("Failed to find id").split("").skip(2).collect();

        // Get anime title
        anime.title = anime_elem.select(&Selector::parse(".film-detail .film-name .dynamic-name").unwrap()).next()
            .expect("Title not found").text().collect::<String>().trim().to_string();

        // Get anime japanese title
        anime.jtitle = anime_elem.select(&Selector::parse(".film-detail .film-name .dynamic-name").unwrap()).next()
            .expect("Japanese title not found").attr("data-jname").expect("Japanese title not found").trim().to_string();

        // Get anime poster url
        anime.poster = anime_elem.select(&Selector::parse(".film-poster .film-poster-img").unwrap()).next()
            .expect("Poster url not found").attr("data-src").expect("Poster url not found").trim().to_string();

        // Get anime duration
        anime.details.duration = anime_elem.select(&Selector::parse(".film-detail .fd-infor .fdi-item.fdi-duration").unwrap()).next()
            .expect("Duration not found").text().collect::<String>().trim().to_string();

        // Get anime type
        anime.details.r#type = anime_elem.select(&Selector::parse(".film-detail .fd-infor .fdi-item:nth-of-type(1)").unwrap()).next()
            .expect("Type not found").text().collect::<String>().trim().to_string();

        // Get anime rating
        if let Some(rating_elem) = anime_elem.select(&Selector::parse(".film-poster .tick-rate").unwrap()).next() {
            anime.details.rating = rating_elem.text().collect::<String>().trim().to_string();
        }

        // Get anime sub episode count
        if let Some(sub_count_elem) = anime_elem.select(&Selector::parse(".film-poster .tick-sub").unwrap()).next() {
            anime.episodes.sub = sub_count_elem.text().collect::<String>().trim().parse().unwrap();
        }

        // Get anime dub episode count
        if let Some(dub_count_elem) = anime_elem.select(&Selector::parse(".film-poster .tick-dub").unwrap()).next() {
            anime.episodes.dub = dub_count_elem.text().collect::<String>().trim().parse().unwrap();
        }

        animes.push(anime);
    }

    return animes;
}

pub fn scrape_top10_anime(top10_wapper: ElementRef, period: &str) -> Vec<Top10AnimeItem> {
    let mut top10_animes = Vec::new();

    for top10_elem in top10_wapper.select(&Selector::parse(format!("#top-viewed-{} ul li", period).as_str()).unwrap()) {
        let mut top10_item = Top10AnimeItem {
            rank: 0,
            id: String::new(),
            title: String::new(),
            jtitle: String::new(),
            poster: String::new(),
            episodes: AnimeEpisodes { sub: 0, dub: 0 }
        };

        // Get top 10 anime rank
        top10_item.rank = top10_elem.select(&Selector::parse(".film-number span").unwrap()).next().expect("Failed to find rank")
            .text().collect::<String>().trim().parse().expect("Failed to parse rank");

        // Get top 10 anime id
        top10_item.id = top10_elem.select(&Selector::parse(".film-detail .dynamic-name").unwrap()).next().expect("Failed to find id")
            .attr("href").expect("Failed to find id").split("").skip(2).collect::<String>().trim().to_string();

        // Get top 10 title
        top10_item.title = top10_elem.select(&Selector::parse(".film-detail .dynamic-name").unwrap()).next()
            .expect("Title not found").text().collect::<String>().trim().to_string();

        // Get top 10 japanese title
        top10_item.jtitle = top10_elem.select(&Selector::parse(".film-detail .dynamic-name").unwrap()).next()
            .expect("Japanese title not found").attr("data-jname").expect("Japanese title not found").trim().to_string();

        // Get top 10 poster url
        top10_item.poster = top10_elem.select(&Selector::parse(".film-poster .film-poster-img").unwrap()).next()
            .expect("Poster url not found").attr("data-src").expect("Poster url not found").trim().to_string();

        // Get top 10 sub episode count
        if let Some(sub_count_elem) = top10_elem.select(&Selector::parse(".film-detail .fd-infor .tick-item.tick-sub").unwrap()).next() {
            top10_item.episodes.sub = sub_count_elem.text().collect::<String>().trim().parse().unwrap();
        }

        // Get top 10 dub episode count
        if let Some(dub_count_elem) = top10_elem.select(&Selector::parse(".film-detail .fd-infor .tick-item.tick-dub").unwrap()).next() {
            top10_item.episodes.dub = dub_count_elem.text().collect::<String>().trim().parse().unwrap();
        }

        top10_animes.push(top10_item);
    }

    return top10_animes;
}