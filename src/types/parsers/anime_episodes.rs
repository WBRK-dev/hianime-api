#[derive(serde::Serialize)]
pub struct AnimeEpisodes {
    pub episodes: Vec<AnimeEpisode>
}

#[derive(serde::Serialize)]
pub struct AnimeEpisode {
    pub title: String,
    pub jtitle: String,
    pub episode_id: String,
    pub number: u8,
    pub is_filler: bool
}

#[derive(serde::Deserialize)]
pub struct EpisodeHtml {
    pub status: bool,
    pub html: String,
    pub totalItems: u8,
    pub continueWatch: Option<String>
}