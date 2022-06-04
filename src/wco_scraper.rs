use std::fs;

use scraper::{Html, Selector};

// is there a better way to do this?  I feel like I'm doing something wrong with this
mod database_handling;

pub fn scrape(url: &str) {
    println!("In file {}", url); // TODO convert this to use downloaded html instead of a local file. Keep local file handling for testing so we don't abuse the servers we scrape from

    let html = fs::read_to_string(url)
        .expect("Something went wrong reading the file");
    let document = Html::parse_document(&html);
    let a_selector = Selector::parse("div.cat-eps > a.sonra").unwrap();
    let showtitle_selector = Selector::parse(".video-title > h1 > div.h1-tag > a").unwrap();
    let mut showtitle:String = "".to_string();
    for show in document.select(&showtitle_selector) {
        showtitle = show.inner_html().to_string();
    }
    let mut a = Vec::new();
    for each in document.select(&a_selector) {
        a.push(each)
    }

    for element in &a { // todo figure out how to get .rev() to work on this without DoubleEndedIterator so that we can reverse order the element items before processing them
        let url = element.value().attr("href").unwrap().to_string();
        let title = element.value().attr("title").unwrap().to_string();
        database_handling::db_add(&showtitle, title, url);
    }

}