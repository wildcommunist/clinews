use std::error::Error;

use dotenv::dotenv;

use newsapi::{Article, Country, Endpoint, NewsAPI};

mod theme;

fn render_articles(articles: &Vec<Article>) {
    let theme = theme::default_theme();
    theme.print_text("# Top headlines\n\n");
    for a in articles.iter() {
        theme.print_text(&format!("`{}`", a.Title()));
        theme.print_text(&format!("> {}", a.URL()));
        theme.print_text("---");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv();

    let api_key = std::env::var("API_KEY")?;
    let mut newsapi = NewsAPI::new(&api_key);
    newsapi.endpoint(Endpoint::TopHeadlines).country(Country::US {});

    let articles = newsapi.fetch_async().await?;

    render_articles(&articles.articles);

    Ok(())
}
