mod routes;
mod parsers;
mod controllers;
mod utils;
mod config;
mod types;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let routes = routes::create();

    let ip: String = utils::env::get("IP", Some(config::default_env::IP)).unwrap();
    let port: u16 = utils::env::get_int("PORT", Some(config::default_env::PORT)).unwrap().try_into().unwrap();

    println!("Running on http://{}:{}", ip, port);

    let ip_parts: [u8; 4] = ip.split(".").map(|part| part.parse::<u8>().unwrap()).collect::<Vec<u8>>().try_into().unwrap();
    warp::serve(routes)
        .run((ip_parts, port))
        .await;
}