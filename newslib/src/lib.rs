use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum NewApiError {
    #[error("Failed fetching articles")]
    RequestFailed(ureq::Error),
    #[error("Failed converting response to string")]
    FailedResonseToString(std::io::Error),
    #[error("Articles parsing failed")]
    ArticleParsingFailed(serde_json::Error)
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

pub fn get_articles(url: &str) -> Result<Articles, NewApiError> {
    let response = ureq::get(url).call().map_err(|e| NewApiError::RequestFailed(e))?.into_string().map_err(
        |e| NewApiError::FailedResonseToString(e)
    )?;
    let articles: Articles = serde_json::from_str(&response).map_err(|e| NewApiError::ArticleParsingFailed(e))?;
    Ok(articles)
}
