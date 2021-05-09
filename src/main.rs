#[macro_use]
extern crate clap;

use std::time::Duration;

use anyhow::Result;
use clap::{value_t, App, Arg};
use reqwest::blocking::ClientBuilder;
use rust_crawler::{crawler::Crawler, LinkExtractor};
use url::Url;

fn main() -> Result<()> {
    env_logger::init();

    let matches = app_from_crate!()
        .arg(
            Arg::with_name("URL")
                .default_value("https://www.rust-lang.org")
                .index(1)
                .help("URL where this program starts crawling"),
        )
        .arg(
            Arg::with_name("MAXIMUM_PAGES")
                .long("maximum-pages")
                .short("n")
                .help("Maximum number of pages to be crawled"),
        )
        .get_matches();

    let url = Url::parse(
        matches
            .value_of("URL")
            .unwrap_or("https://www.rust-lang.org"),
    )?;
    let max_pages = value_t!(matches.value_of("MAXIMUM_PAGES"), usize).unwrap_or(10);
    let client = ClientBuilder::new().build()?;
    let extractor = LinkExtractor::from_client(client);
    let crawler = Crawler::new(&extractor, url);
    let wait = Duration::from_millis(1000);

    for url in crawler.take(max_pages) {
        println!("{}", url);
        std::thread::sleep(wait.clone());
    }

    Ok(())
}
