use headless_chrome::{
    protocol::cdp::{self},
    Browser, LaunchOptionsBuilder,
};
use log::info;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};

use reqwest::Error;

pub async fn scrape_headless(url: &String, selector: &String) -> Result<String, anyhow::Error> {
    let browser = Browser::new(LaunchOptionsBuilder::default().headless(false).build()?)?;

    let tab = browser.new_tab()?;
    tab.call_method(cdp::DOM::Enable {
        0: Some(serde_json::Value::Null),
    })?;
    tab.call_method(cdp::CSS::Enable {
        0: Some(serde_json::Value::Null),
    })?;

    tab.navigate_to(url)?.wait_until_navigated()?;
    tab.wait_for_element(selector)?;

    let evaluated = tab.evaluate(
        format!(
            r#" 



            let elem = document.querySelector("{}");
                if (elem) {{
                    let computedStyle = window.getComputedStyle(elem);
                    let outputDiv = document.createElement("div");

                    outputDiv.innerHTML = elem.outerHTML;

                    // for (let i = 0; i < computedStyle.length; i++) {{
                    //     let property = computedStyle[i];
                    //     let value = computedStyle.getPropertyValue(property);
                    //     outputDiv.style.setProperty(property, value);
                    // }}

                    let doc = document.implementation.createHTMLDocument("new doc");
                    let base = document.createElement("base");
                    base.href = baseURL; // Set the base URL

                    doc.head.appendChild(base);
                    doc.body.appendChild(outputDiv);

                    doc.documentElement.outerHTML;

                }} else {{
                    null;
                }}"#,
            selector
        )
        .as_str(),
        false,
    );

    // Handle the result gracefully
    let outer_html = match evaluated {
        Ok(result) => {
            // Extract the value from the result, if present
            if let Some(o) = result.value {
                println!("Evaluated Output: {}", o.as_str().unwrap_or("No output"));

                o
            } else {
                panic!("Element not found or no output generated.");
            }
        }
        Err(e) => {
            panic!("An error occurred: {}", e);
        }
    };

    Ok(outer_html
        .to_string()
        .strip_prefix('"')
        .unwrap()
        .strip_suffix('"')
        .unwrap()
        .to_string())
}

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

    // Ok(document
    //     .select(&selector)
    //     .flat_map(|el| el.text())
    //     .collect())

    Ok(document.select(&selector).next().unwrap().html())

    // match r#type {
    //     ScrapeType::Text => document
    //         .select(&selector)
    //         .flat_map(|el| el.text())
    //         .collect(),
    //     ScrapeType::Html => document.select(&selector).next().unwrap().html(),
    // };
}
