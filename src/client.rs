use std::io::Read;

use hyper::Client as WebClient;
use hyper::header::Headers;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use serde_json;

header! { (Authorization, "Authorization") => [String] }

pub struct Client<'a> {
    url: &'a str,
    api_key: &'a str,
    web_client: WebClient,
}

#[derive(Serialize, Deserialize)]
pub struct AccountInfo {
    pub id: String,
    pub tags: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct Accounts {
    accounts: Vec<AccountInfo>
}

impl<'a> Client<'a> {
    pub fn new(url: &'a str, api_key: &'a str) -> Client<'a> {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let web_client = WebClient::with_connector(connector);

        Client {
            url: url,
            api_key: api_key,
            web_client: web_client
        }
    }

    // Get Account list for current auth token
    pub fn accounts(&self) -> Vec<AccountInfo> {
        let input = self.get("accounts");
        let result: Accounts = serde_json::from_str(&input).unwrap();

        result.accounts
    }

    fn get(&self, params: &str) -> String {
        let mut res = String::new();
        let mut headers = Headers::new();

        headers.set(Authorization(format!("Bearer {}", self.api_key)));

        self.web_client
            .get(&format!("{}/{}", self.url, params))
            .headers(headers)
            .send()
            .unwrap()
            .read_to_string(&mut res)
            .unwrap();

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // # TODO: Move integration tests to `tests/`
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
