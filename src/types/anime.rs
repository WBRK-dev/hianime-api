#[derive(serde::Serialize)]
pub struct Anime {
    pub id: String,
    pub name: String,
    pub jname: String,
    pub poster: String,
    pub details: AnimeDetails,
    pub episodes: AnimeEpisodes
}

#[derive(serde::Serialize)]
pub struct AnimeEpisodes {
    pub sub: u32,
    pub dub: u32
}

#[derive(serde::Serialize)]
pub struct AnimeDetails {
    pub duration: String,
    pub r#type: String,
    pub rating: String
}