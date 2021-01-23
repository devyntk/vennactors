use serde::Deserialize;
use std::fmt::Debug;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Configuration {
    change_keys: Vec<String>,
    pub images: ImageConfig
}

#[derive(Deserialize, Debug, Clone)]
pub struct ImageConfig {
    base_url: String,
    pub(crate) secure_base_url: String,
    backdrop_sizes: Vec<String>,
    logo_sizes: Vec<String>,
    poster_sizes: Vec<String>,
    profile_sizes: Vec<String>,
    still_sizes: Vec<String>,
}
