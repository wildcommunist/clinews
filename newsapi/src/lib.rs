use std::error::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub author: Option<String>,
    pub title: String,
    pub url: String,
    pub content: Option<String>,
    pub description: Option<String>,
    pub url_to_image: Option<String>,
}

pub fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    let articles = serde_json::from_str(&response)?;

    Ok(articles)
}