use crate::api_structs::*;
use reqwest::Error;
use log::debug;

pub(crate) struct TMDBClient {
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
}
