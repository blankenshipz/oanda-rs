extern crate ratelimit;

use std::error::Error;
use std::time::Duration;

use chrono::DateTime;
use chrono::Utc;

use hyper::client::HttpConnector;
use hyper::{client::Builder as WebClientBuilder, Client as WebClient, Request};
use hyper_tls::HttpsConnector;
use ratelimit::Ratelimiter;

use serde_json;

use crate::account::{Account, Accounts};
use crate::instrument::pricing_query::PricingQuery;

pub struct Client<'a> {
    url: &'a str,
    api_key: &'a str,
    client: WebClient<HttpsConnector<HttpConnector>>,
    rate_limiter: Ratelimiter,
}

/// The Client facilitates all requests to the Oanda API
impl<'a> Client<'a> {
    pub fn new(url: &'a str, api_key: &'a str) -> Client<'a> {
        let mut http = HttpConnector::new();
        http.set_nodelay(true);
        http.set_reuse_address(true);
        http.set_keepalive(Some(Duration::from_secs(120)));
        http.enforce_http(false);

        let connector = HttpsConnector::new_with_connector(http);
        let client = WebClientBuilder::default().build(connector);

        // Client is allowed to have no more than 120 requests per second on average,
        // with bursts of no more than 60 requests. Excess requests will be
        // rejected. This restriction is applied for each access token,
        // not for each individual connection.
        let capacity = 60; // number of tokens the bucket will hold
        let quantum = 1; // number of tokens to add per interval
        let interval = Duration::from_millis(9); // interval between token adds
        let rate_limiter = Ratelimiter::new(capacity, quantum, interval.as_millis() as u64);

        let client = Client {
            url,
            api_key,
            client,
            rate_limiter,
        };

        client
    }

    /// Get Account list for current auth token
    pub async fn accounts(&self) -> Result<Vec<Account>, Box<dyn Error>> {
        let input = self.get("accounts").await?;
        let mut result: Accounts = serde_json::from_str(&input).unwrap();

        for x in result.accounts.iter_mut() {
            x.client = Some(&self);
        }

        Ok(result.accounts)
    }

    pub fn pricing_for(&self, instrument: String, from: DateTime<Utc>) -> PricingQuery {
        PricingQuery::new(&self, instrument, from)
    }

    pub async fn wait_for_rate_limiter(&self) {
        while self.rate_limiter.try_wait().is_err() {
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    }

    pub async fn get(&self, params: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.wait_for_rate_limiter().await;

        let web_client = &self.client;

        let url = format!("{}/{}", self.url, params);
        let request = Request::get(url)
            .header("Content-Type", "application/json")
            .header("Accept-Datetime-Format", "RFC3339")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(hyper::Body::from(""))?;

        let res = web_client.request(request).await?;
        let res = hyper::body::to_bytes(res.into_body()).await?;
        let res = String::from_utf8(res.to_vec())?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    /// # TODO: Move integration tests to `tests/`
    #[test]
    fn it_can_read_accounts() {
        let url = env::var("OANDA_API_URL").unwrap();
        let key = env::var("OANDA_API_KEY").unwrap();
        let account_id = env::var("OANDA_TEST_ACCOUNT_ID").unwrap();

        let client = Client::new(&url, &key);
        let info = client.accounts();

        assert_eq!(info.iter().any(|ref x| x.id == account_id), true)
    }
}
