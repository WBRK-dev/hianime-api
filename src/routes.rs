use std::collections::HashMap;
use warp::Filter;

use crate::controllers;

pub fn create() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_home_page().or(get_anime_episodes())
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