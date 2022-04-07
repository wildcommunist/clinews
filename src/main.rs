use std::error::Error;
use std::fmt::format;

use colour::{dark_cyan, dark_green};
use dotenv::dotenv;

use newsapi::{Articles, get_articles};

fn render_articles(articles: &Articles) {
    for a in &articles.articles {
        dark_green!("> {}\n",a.title);
        dark_cyan!(" - {}\n\n",a.url)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;
    let api_key = std::env::var("API_KEY")?;
    let url = format!("{}{}", "https://newsapi.org/v2/top-headlines?country=us&apiKey=", api_key);
    let articles = get_articles(url.as_str())?;

    render_articles(&articles);

    Ok(())
}
