#[derive(serde::Serialize)]
pub struct HomePage {
    pub spotlight: Vec<SpotlightItem>,
    // trending: Vec<TrendingItem>,
    // latest_episodes: Vec<LatestEpisodeItem>,
    // top_upcoming: Vec<TopUpcomingItem>,
    // top10: Top10AnimeWrapper,
    // top_airing: Vec<TopAiringItem>,
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
    pub episodes: crate::types::anime::AnimeEpisodes
}

#[derive(serde::Serialize)]
pub struct TrendingItem {
    pub rank: u8,
    pub id: String,
    pub title: String,
    // pub jname: String,
    pub poster: String,
}

#[derive(serde::Serialize)]
pub struct LatestEpisodeItem {
    pub id: String,
    pub title: String,
    // pub jname: String,
    pub poster: String,
    pub details: crate::types::anime::AnimeDetails,
    pub episodes: crate::types::anime::AnimeEpisodes
}

#[derive(serde::Serialize)]
pub struct TopUpcomingItem {
    pub id: String,
    pub title: String,
    // pub jname: String,
    pub poster: String,
    pub details: crate::types::anime::AnimeDetails,
    pub episodes: crate::types::anime::AnimeEpisodes
}

#[derive(serde::Serialize)]
pub struct Top10AnimeWrapper {
    today: Vec<Top10AnimeItem>,
    week: Vec<Top10AnimeItem>,
    month: Vec<Top10AnimeItem>
}

#[derive(serde::Serialize)]
pub struct Top10AnimeItem {
    pub rank: u8,
    pub id: String,
    pub title: String,
    // pub jname: String,
    pub poster: String,
    pub episodes: crate::types::anime::AnimeEpisodes
}

#[derive(serde::Serialize)]
pub struct TopAiringItem {
    pub id: String,
    pub title: String,
    pub jtitle: String,
    pub poster: String,
}