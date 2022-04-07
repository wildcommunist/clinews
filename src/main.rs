use std::error::Error;

use colour::{dark_cyan, dark_green};
use newsapi::{Articles, get_articles};

fn render_articles(articles: &Articles) {
    for a in &articles.articles {
        dark_green!("> {}\n",a.title);
        dark_cyan!(" - {}\n\n",a.url)
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=8ee50aaee4154bbf84c7477998689233";
    let articles = get_articles(url)?;

    render_articles(&articles);

    Ok(())
}
