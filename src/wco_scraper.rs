use std::fs;
use std::path::Path;

use async_trait::async_trait;
use scraper::{Html, Selector};

// is there a better way to do this?  I feel like I'm doing something wrong with this
mod database_handling;

#[async_trait]
trait Scraper {
    async fn scrape(&self);
}

#[async_trait]
impl scrape for Crawler {
    async fn eatmybutt(&self {})
}

#[async_trait]
async fn scrape(url: String) {
    let x = Crawler { url: url };
}

#[derive(Default)]
struct Crawler {
    url: String,
}

#[async_trait]
impl Scraper for Crawler {
    async fn scrape(&self) {
        let html =
            fs::read_to_string(Path::new(self)).expect("Something went wrong reading the file");
        let document = Html::parse_document(&html);
        let a_selector = Selector::parse("div.cat-eps > a.sonra").unwrap();
        let showtitle_selector = Selector::parse(".video-title > h1 > div.h1-tag > a").unwrap();
        let mut seriestitle: String = "".to_string();
        for series in document.select(&showtitle_selector) {
            seriestitle = series.inner_html().to_string();
        }
        let mut a = Vec::new();
        for each in document.select(&a_selector) {
            a.push(each);
        }

        for element in &a {
            let url = element.value().attr("href").unwrap().to_string();
            let title = element.value().attr("title").unwrap().to_string();

            database_handling::db_add(&seriestitle, title, url).await;
            return element;
        }
        return ();
        //TODO this needs to return a future
    }
}
