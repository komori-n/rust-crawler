pub mod crawler;

use reqwest::blocking::Client;

use anyhow::Result;
use select::document::Document;
use select::predicate::Name;
use thiserror::Error;
use url::ParseError as UrlParseError;
use url::Url;

pub struct LinkExtractor {
    client: Client,
}

#[derive(Error, Debug)]
pub enum GetLinksError {
    #[error("Failed to send a request")]
    SendRequest(#[source] reqwest::Error),
    #[error("Failed to read the response body")]
    ResponseBody(#[source] reqwest::Error),
    #[error("Failed to make the link URL absolute")]
    AbsolutizeUrl(#[source] url::ParseError),
    #[error("Server returned an error")]
    ServerError(#[source] reqwest::Error),
}

impl LinkExtractor {
    pub fn from_client(client: Client) -> Self {
        Self { client }
    }

    pub fn get_links(&self, url: Url) -> Result<Vec<Url>, GetLinksError> {
        log::info!(r#"GET "{}""#, url);
        let response = self
            .client
            .get(url)
            .send()
            .map_err(GetLinksError::SendRequest)?;
        let response = response
            .error_for_status()
            .map_err(GetLinksError::ServerError)?;
        let base_url = response.url().clone();
        let status = response.status();
        let header = response.headers().clone();
        let body = response.text().map_err(GetLinksError::ResponseBody)?;
        let doc = Document::from(body.as_str());
        let mut links = Vec::new();
        log::info!(r#"Retrieved {} "{}""#, status, base_url);
        log::debug!(r#"headers {:?}"#, header);

        for href in doc.find(Name("a")).filter_map(|a| a.attr("href")) {
            match Url::parse(href) {
                Ok(mut url) => {
                    url.set_fragment(None);
                    links.push(url);
                }
                Err(UrlParseError::RelativeUrlWithoutBase) => {
                    let url = base_url
                        .join(href)
                        .map(|mut x| {
                            x.set_fragment(None);
                            x
                        })
                        .map_err(GetLinksError::AbsolutizeUrl)?;
                    links.push(url);
                }
                Err(e) => println!("Error: {}", e),
            }
        }

        Ok(links)
    }
}
