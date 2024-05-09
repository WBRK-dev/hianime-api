use crate::types::errors::DefaultError;

pub fn main(err: DefaultError) -> warp::reply::WithStatus<warp::reply::Json> {
    warp::reply::with_status(warp::reply::json(&err), warp::http::StatusCode::INTERNAL_SERVER_ERROR)
}