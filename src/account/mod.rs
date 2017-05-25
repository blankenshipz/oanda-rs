pub mod details;
pub mod instruments;
pub mod summary;

use serde_json;

use client::Client;
use self::details::AccountDetails;
use self::details::Details;
use self::summary::AccountSummary;
use self::summary::Summary;
use self::instruments::AccountInstruments;
use self::instruments::Instrument;

fn none() -> Option<&'static Client<'static>> { None }

#[derive(Deserialize)]
pub struct Accounts<'a> {
    pub accounts: Vec<Account<'a>>
}

#[derive(Deserialize)]
pub struct Account<'a> {
    pub id: String,
    pub tags: Vec<String>,
    #[serde(default = "none")]
    #[serde(skip_deserializing)]
    pub client: Option<&'a Client<'a>>
}

impl <'a>Account<'a> {
    pub fn details(&self) -> Details {
        let input = self.client().get(format!("accounts/{}", self.id).as_str());
        let result: AccountDetails = serde_json::from_str(&input).unwrap();

        result.account
    }

    pub fn instruments(&self) -> Vec<Instrument> {
        let input = self.client().get(
            format!("accounts/{}/instruments", self.id).as_str()
        );
        let result: AccountInstruments = serde_json::from_str(&input).unwrap();

        result.instruments
    }

    pub fn summary(&self) -> Summary {
        let input = self.client().get(
            format!("accounts/{}/summary", self.id).as_str()
        );
        let result: AccountSummary = serde_json::from_str(&input).unwrap();

        result.account
    }

    fn client(&self) -> &'a Client<'a> {
        self.client.expect("Account cannot refer to a client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // # TODO: Move integration tests to `tests/`
    #[test]
    fn it_can_read_account_details() {
        let url = env::var("OANDA_API_URL").unwrap();
        let key = env::var("OANDA_API_KEY").unwrap();
        let account_id = env::var("OANDA_TEST_ACCOUNT_ID").unwrap();

        let client = Client::new(&url, &key);
        let accounts = client.accounts();

        let account = accounts.first().unwrap().clone();
        let details = account.details();

        assert_eq!(details.alias.unwrap(), "Testv20")
    }

    #[test]
    fn it_can_read_account_summary() {
        let url = env::var("OANDA_API_URL").unwrap();
        let key = env::var("OANDA_API_KEY").unwrap();
        let account_id = env::var("OANDA_TEST_ACCOUNT_ID").unwrap();

        let client = Client::new(&url, &key);
        let accounts = client.accounts();

        let account = accounts.first().unwrap().clone();
        let summary = account.summary();

        assert_eq!(summary.alias.unwrap(), "Testv20")
    }

    #[test]
    fn it_can_read_account_instruments() {
        let url = env::var("OANDA_API_URL").unwrap();
        let key = env::var("OANDA_API_KEY").unwrap();
        let account_id = env::var("OANDA_TEST_ACCOUNT_ID").unwrap();

        let client = Client::new(&url, &key);
        let accounts = client.accounts();

        let account = accounts.first().unwrap().clone();
        let instruments = account.instruments();
        // DKK just hapepns to the be the first one, the result here is a list
        // of all USD_* tradable currencies because the test account is USD
        assert_eq!(instruments.first().unwrap().name, "USD_DKK")
    }
}
