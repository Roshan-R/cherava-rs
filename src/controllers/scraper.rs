use log::info;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};

use reqwest::Error;

pub async fn scrape(url: &String, selector: &String) -> Result<String, Error> {
    let client = reqwest::Client::new();

    let html = client
        .get(url)
        .header(
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:129.0) Gecko/20100101 Firefox/129.0",
        )
        .send()
        .await?
        .text()
        .await?;

    info!("Scaping site {} with selector {}", url, selector);
    let document = Html::parse_document(html.as_str());
    let selector = Selector::parse(selector.as_str()).unwrap();

    Ok(document
        .select(&selector)
        .flat_map(|el| el.text())
        .collect())

    // match r#type {
    //     ScrapeType::Text => document
    //         .select(&selector)
    //         .flat_map(|el| el.text())
    //         .collect(),
    //     ScrapeType::Html => document.select(&selector).next().unwrap().html(),
    // };
}
