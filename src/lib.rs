mod error;

use http::{Request, StatusCode, Version};
use hyper::body::Buf;
use hyper_rustls::HttpsConnector;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::io::Read;

pub use error::Error;

pub type SimplePrice = HashMap<String, Decimal>;
pub type SimplePrices = HashMap<String, SimplePrice>;

const ENDPOINT: &str = "https://api.coingecko.com/api/v3/simple/price";

#[derive(Clone, Debug)]
pub struct SimplePriceRequest {
    pub ids: Vec<String>,
    pub vs_currencies: Vec<String>,
    pub include_market_cap: Option<bool>,
    pub include_24hr_vol: Option<bool>,
    pub include_24hr_change: Option<bool>,
    pub include_last_updated_at: Option<bool>,
}

impl SimplePriceRequest {
    fn get_url(&self) -> String {
        let ids = self.ids.join(",");
        let vs_currencies = self.vs_currencies.join(",");
        let mut query = format!("ids={}&vs_currencies={}", ids, vs_currencies);
        if let Some(include_market_cap) = self.include_market_cap {
            query += &format!("&include_market_cap={}", include_market_cap);
        }
        if let Some(include_24hr_change) = self.include_24hr_change {
            query += &format!("&include_24hr_change={}", include_24hr_change);
        }
        if let Some(include_last_updated_at) = self.include_last_updated_at {
            query += &format!("&include_last_updated_at={}", include_last_updated_at);
        }
        format!("{}?{}", ENDPOINT, query)
    }
}

pub async fn simple_price(request: SimplePriceRequest) -> Result<SimplePrices, Error> {
    let uri = request.get_url();
    let request = Request::builder()
        .version(Version::HTTP_2)
        .method("GET")
        .uri(uri)
        .body(hyper::Body::empty())
        .unwrap();
    let client = hyper::Client::builder()
        .http2_only(true)
        .build(HttpsConnector::with_native_roots());
    let response = client.request(request).await?;
    let response_status = response.status().clone();
    let mut response_body = String::new();
    hyper::body::aggregate(response.into_body())
        .await?
        .reader()
        .read_to_string(&mut response_body)?;
    if response_status == StatusCode::OK {
        Ok(serde_json::from_str(&response_body)?)
    } else {
        Err(Error::Gateway {
            status_code: response_status.as_u16(),
            reason: response_body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_price() {
        let request = SimplePriceRequest {
            ids: vec!["ethereum".into()],
            vs_currencies: vec!["usd".into()],
            include_market_cap: None,
            include_24hr_vol: None,
            include_24hr_change: None,
            include_last_updated_at: None,
        };
        let response = simple_price(request).await;

        match response {
            Ok(prices) => {
                println!("{:?}", prices)
            }
            Err(e) => panic!("unexpected error: {:?}", e),
        };
    }
}
