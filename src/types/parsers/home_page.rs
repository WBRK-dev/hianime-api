use crate::types::*;

#[derive(serde::Serialize)]
pub struct HomePage {
    pub spotlight: Vec<SpotlightItem>,
    pub trending: Vec<TrendingItem>,
    pub latest_episodes: Vec<anime::Anime>,
    pub top_upcoming: Vec<anime::Anime>,
    pub top10: Top10AnimeWrapper,
    pub top_airing: Vec<TopAiringItem>,
}

#[derive(serde::Serialize)]
pub struct SpotlightItem {
    pub rank: u8,
    pub id: String,
    pub title: String,
    pub jtitle: String,
    pub description: String,
    pub poster: String,
    pub details: Vec<String>,
    pub episodes: anime::AnimeEpisodes
}

#[derive(serde::Serialize)]
pub struct TrendingItem {
    pub rank: u8,
    pub id: String,
    pub title: String,
    pub jtitle: String,
    pub poster: String,
}

#[derive(serde::Serialize)]
pub struct Top10AnimeWrapper {
    pub day: Vec<Top10AnimeItem>,
    pub week: Vec<Top10AnimeItem>,
    pub month: Vec<Top10AnimeItem>
}

#[derive(serde::Serialize)]
pub struct Top10AnimeItem {
    pub rank: u8,
    pub id: String,
    pub title: String,
    pub jtitle: String,
    pub poster: String,
    pub episodes: anime::AnimeEpisodes
}

#[derive(serde::Serialize)]
pub struct TopAiringItem {
    pub id: String,
    pub title: String,
    pub jtitle: String,
    pub poster: String,
    pub details: Vec<String>
}