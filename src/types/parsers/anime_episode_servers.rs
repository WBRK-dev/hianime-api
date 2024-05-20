#[derive(serde::Serialize)]
pub struct AnimeEpisodeServers {
    pub sub: Vec<AnimeEpisodeServer>,
    pub dub: Vec<AnimeEpisodeServer>,
    pub raw: Vec<AnimeEpisodeServer>
}

#[derive(serde::Serialize)]
pub struct AnimeEpisodeServer {
    pub name: String,
    pub id: u8
}

#[derive(serde::Deserialize)]
pub struct EpisodeServersHtml {
    pub status: bool,
    pub html: String,
}