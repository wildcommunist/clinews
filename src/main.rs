use std::error::Error;

use dotenv::dotenv;

use newsapi::{Articles, get_articles};

mod theme;

fn render_articles(articles: &Articles) {
    let theme = theme::default_theme();
    theme.print_text("# Top headlines\n\n");
    for a in &articles.articles {
        theme.print_text(&format!("`{}`", a.title));
        theme.print_text(&format!("  *{}*", a.url));
        theme.print_text("---");
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
