use std::io::Read;

use hyper::Client as WebClient;
use hyper::header::Headers;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use serde_json;

use account::details::*;
use account::*;

header! { (Authorization, "Authorization") => [String] }
header! { (AcceptDatetimeFormat, "AcceptDatetimeFormat") => [String] }

pub struct Client<'a> {
    url: &'a str,
    api_key: &'a str,
    web_client: WebClient,
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
    pub fn accounts(&self) -> Vec<Account> {
        let input = self.get("accounts");
        let mut result: Accounts = serde_json::from_str(&input).unwrap();

        for x in result.accounts.iter_mut() {
            x.client = Some(&self);
        }

        result.accounts
    }

    pub fn account_details(&self, account: &Account) -> Details {
        let input = self.get(format!("accounts/{}", account.id).as_str());
        let mut result: AccountDetails = serde_json::from_str(&input).unwrap();

        result.account
    }

    fn get(&self, params: &str) -> String {
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

        headers
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
