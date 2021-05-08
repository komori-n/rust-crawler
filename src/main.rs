use anyhow::Result;
use reqwest::blocking::ClientBuilder;
use rust_crawler::LinkExtractor;
use url::Url;

fn main() -> Result<()> {
    let url = Url::parse("https://www.rust-lang.org")?;
    let client = ClientBuilder::new().build()?;
    let extractor = LinkExtractor::from_client(client);

    let links = extractor.get_links(url)?;
    for link in links.iter() {
        println!("{}", link);
    }

    Ok(())
}
