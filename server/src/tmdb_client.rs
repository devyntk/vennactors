use crate::api_structs::*;
use reqwest::Error;
use log::debug;

#[derive(Clone)]
pub struct TMDBClient {
    api_key: String,
    client: reqwest::Client,
    image_url: Option<String>
}

impl TMDBClient {
    pub fn new(api_key: String, client: reqwest::Client) -> TMDBClient {
        TMDBClient {
            api_key,
            client,
            image_url: None
        }
    }

    pub async fn get_config(&mut self) -> Result<Configuration, Error> {
        let url = format!("https://api.themoviedb.org/3/configuration?api_key={}", self.api_key);
        debug!("{}",url);

        let resp = self.client
            .get(&url)
            .send()
            .await?
            .json::<Configuration>()
            .await?;

        self.image_url = Some(resp.clone().images.secure_base_url);

        Ok(resp)
    }

    pub async fn multi_search(&mut self, search: String) -> Result<MultiSearch, Error> {
        let url = format!("https://api.themoviedb.org/3/search/multi\
        ?api_key={}&query={}&include_adult=false", self.api_key, search);
        debug!("{}",url);

        let resp = self.client
            .get(&url)
            .send()
            .await?;

        let obj = resp
            .json::<MultiSearch>()
            .await?;

        Ok(obj)
    }

    pub async fn movie_credits(&mut self, id: u32) -> Result<MovieCredits, Error> {
        let url = format!("https://api.themoviedb.org/3/movie/{}/credits\
        ?api_key={}", id, self.api_key);
        debug!("{}",url);

        let resp = self.client
            .get(&url)
            .send()
            .await?;

        let obj = resp
            .json::<MovieCredits>()
            .await?;

        Ok(obj)
    }

    pub async fn tv_credits(&mut self, id: u32) -> Result<TVCredits, Error> {
        let url = format!("https://api.themoviedb.org/3/tv/{}/aggregate_credits\
        ?api_key={}", id, self.api_key);
        debug!("{}",url);

        let resp = self.client
            .get(&url)
            .send()
            .await?;

        let obj = resp
            .json::<TVCredits>()
            .await?;

        Ok(obj)
    }
}
