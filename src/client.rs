extern crate ratelimit;

use std::io::Read;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::SyncSender;

use chrono::datetime::DateTime;
use chrono::UTC;

use hyper::Client as WebClient;
use hyper::header::Headers;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use serde_json;

use account::*;
use account::details::*;
use account::summary::*;

use instrument::pricing_query::PricingQuery;

header! { (Authorization, "Authorization") => [String] }
header! { (AcceptDatetimeFormat, "AcceptDatetimeFormat") => [String] }
header! { (Connection, "Connection") => [String] }

pub struct Client<'a> {
    url: &'a str,
    api_key: &'a str,
    web_client: WebClient,
    sender: SyncSender<()>
}

/// The Client facilitates all requests to the Oanda API
impl<'a> Client<'a> {
    pub fn new(url: &'a str, api_key: &'a str) -> Client<'a> {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let web_client = WebClient::with_connector(connector);

        /// Client is allowed to have no more than 120 requests per second on average,
        /// with bursts of no more than 60 requests. Excess requests will be
        /// rejected. This restriction is applied for each access token,
        /// not for each individual connection.
        let mut ratelimit = ratelimit::Ratelimit::configure()
            .capacity(60) // number of tokens the bucket will hold
            .quantum(1)   // add one token per interval
            .interval(Duration::from_millis(9)) // TODO: allows for 111 per second 8.3 would be 120 (Quantum per 9 milliseconds)
            .build();

        let client = Client {
            url: url,
            api_key: api_key,
            web_client: web_client,
            sender: ratelimit.clone_sender()
        };

        thread::spawn(move || {
            loop {
                ratelimit.run();
            }
        });

        client
    }

    /// Get Account list for current auth token
    pub fn accounts(&self) -> Vec<Account> {
        let input = self.get("accounts");
        let mut result: Accounts = serde_json::from_str(&input).unwrap();

        for x in result.accounts.iter_mut() {
            x.client = Some(&self);
        }

        result.accounts
    }

    pub fn pricing_for(&self, instrument: String, from: DateTime<UTC>) -> PricingQuery {
        PricingQuery::new(&self, instrument, from)
    }

    pub fn get(&self, params: &str) -> String {
        self.sender.send(());

        let mut res = String::new();

        self.web_client
            .get(&format!("{}/{}", self.url, params))
            .headers(self.headers())
            .send()
            .unwrap()
            .read_to_string(&mut res)
            .unwrap();

        res
    }

    fn headers(&self) -> Headers {
        let mut headers = Headers::new();

        headers.set(Authorization(format!("Bearer {}", self.api_key)));
        headers.set(AcceptDatetimeFormat("RFC3339".to_string()));
        headers.set(Connection("Keep-Alive".to_string()));

        headers
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
