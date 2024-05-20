use std::collections::HashMap;
use warp::Filter;

use crate::controllers;

pub fn create() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_home_page()
        .or(get_anime_episodes())
        .or(get_anime_episode_servers())
}


// GET - Home page
fn get_home_page() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("home")
        .and(warp::get())
        .and_then(controllers::home_page::main)
}

// GET - Anime Episodes
fn get_anime_episodes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("episodes")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and_then(controllers::anime_episodes::main)
}

// GET - Anime Episode Servers
fn get_anime_episode_servers() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("episode-servers")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and_then(controllers::anime_episode_servers::main)
}