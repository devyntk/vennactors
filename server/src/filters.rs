use crate::tmdb_client::TMDBClient;
use crate::handlers::*;
use warp::{Filter, path, Rejection, Reply};
use std::convert::Infallible;
use warp::http::StatusCode;
use shared::ErrorMessage;

pub fn filters(client: TMDBClient) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path("api").and(
        search(client.clone())
            .recover(handle_api_rejection)
        )
}


fn with_client(client: TMDBClient) -> impl Filter<Extract = (TMDBClient,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}

fn search(client: TMDBClient) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("search" / String)
        .and(warp::get())
        .and(with_client(client))
        .and_then(search_handler)
}

pub async fn handle_api_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
