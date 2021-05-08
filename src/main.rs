#[macro_use]
extern crate clap;

use anyhow::Result;
use clap::{App, Arg};
use reqwest::blocking::ClientBuilder;
use rust_crawler::LinkExtractor;
use url::Url;

fn main() -> Result<()> {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("URL")
                .default_value("https://www.rust-lang.org")
                .index(1),
        )
        .get_matches();

    let url = Url::parse(
        matches
            .value_of("URL")
            .unwrap_or("https://www.rust-lang.org"),
    )?;
    let client = ClientBuilder::new().build()?;
    let extractor = LinkExtractor::from_client(client);

    let links = extractor.get_links(url)?;
    for link in links.iter() {
        println!("{}", link);
    }

    Ok(())
}
