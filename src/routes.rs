use warp::Filter;

use crate::controllers;

pub fn create() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_home_page()
}


// GET - Test data
fn get_home_page() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("home")
        .and(warp::get())
        .and_then(controllers::home_page::main)
}