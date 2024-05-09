use std::convert::Infallible;

pub async fn main() -> Result<warp::reply::WithStatus<impl warp::Reply>, Infallible> {

    let parser = crate::parsers::home_page::get().await;
    match parser {
        Ok(reply) => Ok(reply),
        Err(err) => Ok(crate::controllers::error::main(err))
    }

}