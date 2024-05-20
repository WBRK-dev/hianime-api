use std::{collections::HashMap, convert::Infallible};
use crate::controllers::error;
use crate::types::errors::DefaultError;

pub async fn main(query: HashMap<String, String>) -> Result<warp::reply::WithStatus<impl warp::Reply>, Infallible> {

    let anime_id = query.get("id");
    if let None = anime_id { return Ok(error::main(DefaultError { message: String::from("value") })); }
    let anime_id = anime_id.unwrap().to_owned();
        
    let parser = crate::parsers::anime_episodes::get(anime_id).await;
    match parser {
        Ok(reply) => Ok(reply),
        Err(err) => Ok(error::main(err))
    }
    
}