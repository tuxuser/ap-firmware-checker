use ureq;
use serde::{Deserialize, Serialize};
use scraper::{Html, Selector};

pub const BASE_URL: &str = "https://www.analogue.co/support/pocket";

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Firmware {
    pub id: String,
    pub published_at: String,
    pub version: String,
    pub details: FirmwareDetails,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FirmwareDetails {
    pub version: String,
    pub filename: String,
    pub filesize: String,
    pub md5_hash: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    pub title: String,
    pub slug: String,
    pub url: Option<String>,
    pub external: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageProps {
    pub product: Product,
    pub firmware: Firmware,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Props {
    pub page_props: PageProps,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NextData {
    pub props: Props,
}

pub fn fetch_firmware_info() -> Result<NextData, Box<dyn std::error::Error>> {
    let response = ureq::get(BASE_URL)
        .call()?;
    let body = response.into_string()?;
    let json_data = parse_json_data(&body).unwrap();
    let next_data: NextData = serde_json::from_str(&json_data).unwrap();
    
    Ok(next_data)
}

fn parse_json_data(html: &str) -> Result<String, Box<dyn std::error::Error>> {
    let fragment = Html::parse_fragment(html);
    let script_selector = Selector::parse("script[id='__NEXT_DATA__']").unwrap();
    let parsed = fragment
        .select(&script_selector)
        .next()
        .expect("Failed to find script tag")
        .text()
        .next()
        .expect("Failed to get text from script tag")
        .to_string();

    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let json_data = include_str!("./sample_data.json");
        let next_data: NextData = serde_json::from_str(&json_data).expect("Failed to parse __NEXT_DATA__");
        assert_eq!(next_data.props.page_props.firmware.version, "1.1-beta-6");
    }

    #[tokio::test]
    async fn live_data() {
        let res = fetch_firmware_info();
        assert!(res.is_ok());
    }
}