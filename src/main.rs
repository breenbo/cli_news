use dotenv::dotenv;
use newsapi::{render, NewsAPI, NewsAPIResponse};
// use newsapi::{get_articles, Articles};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let api_key = std::env::var("API_KEY")?;

    let mut newsapi = NewsAPI::new(&api_key);
    newsapi
        .endpoint(newsapi::Endpoint::TopHeadLines)
        .country(newsapi::Country::Fr);

    let articles: NewsAPIResponse = newsapi.async_fetch().await?;
    render(&articles.articles);
    // sync method
    // let articles: NewsAPIResponse = newsapi.fetch()?;
    // articles.render();

    Ok(())
}
