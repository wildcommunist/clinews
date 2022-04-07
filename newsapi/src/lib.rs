use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Failed to fetch articles")]
    RequestFailed(ureq::Error),

    #[error("Failed to convert response to string")]
    ConversionFailed(std::io::Error),

    #[error("Atricle parsing failed")]
    ResponseJsonFailed(serde_json::Error),
}

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

pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    let response = ureq::get(url)
        .call().map_err(|e| NewsApiError::RequestFailed(e))?
        .into_string().map_err(|e| NewsApiError::ConversionFailed(e))?;

    let articles = serde_json::from_str(&response)
        .map_err(|e| NewsApiError::ResponseJsonFailed(e))?;

    Ok(articles)
}