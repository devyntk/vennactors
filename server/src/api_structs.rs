use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Deserialize, Debug, Clone)]
pub struct Configuration {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MultiSearch {
    results: Vec<SearchResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "media_type")]
#[serde(rename_all = "lowercase")]
pub enum SearchResult {
    Movie(MovieSearchResult),
    TV(TVSearchResult),
    Person(PersonSearchResult)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MovieSearchResult {
    poster_path: Option<String>,
    backdrop_path: Option<String>,
    id: u32,
    overview: String,
    title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TVSearchResult {
    poster_path: Option<String>,
    backdrop_path: Option<String>,
    id: u32,
    overview: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PersonSearchResult {
    profile_path: Option<String>,
    id: u32,
    name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MovieCredits {
    cast: Vec<CastCredits>,
    crew: Vec<CrewCredits>
}
#[derive(Deserialize, Debug, Clone)]
pub struct CastCredits {
    id: u32,
    name: String,
    profile_path: Option<String>,
    character: Option<String>,

}
#[derive(Deserialize, Debug, Clone)]
pub struct CrewCredits {
    id: u32,
    name: String,
    profile_path: Option<String>,
    job: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct TVCredits {
    cast: Vec<TVCastCredits>,
    crew: Vec<TVCrewCredits>
}
#[derive(Deserialize, Debug, Clone)]
pub struct TVCastCredits {
    id: u32,
    name: String,
    profile_path: Option<String>,
    roles: Vec<Role>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Role {
    character: String,
    episode_count: u16
}

#[derive(Deserialize, Debug, Clone)]
pub struct TVCrewCredits {
    id: u32,
    name: String,
    profile_path: Option<String>,
    jobs: Vec<Job>
}
#[derive(Deserialize, Debug, Clone)]
pub struct Job {
    job: String,
    episode_count: u16
}
