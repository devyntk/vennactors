use std::convert::Infallible;
use crate::tmdb_client::TMDBClient;

pub async fn search_handler(search: String, mut client: TMDBClient) -> Result<impl warp::Reply, Infallible> {
    return match client.multi_search(search.clone()).await {
        Ok(result) => {
            Ok(warp::reply::json(&result))
        }
        Err(_err) => {
            Ok(warp::reply::json(&search))
        }
    }
}
