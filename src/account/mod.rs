pub mod details;

use client::Client;
use self::details::Details;

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
        self.client().account_details(&self)
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
        let account = accounts.first().unwrap();
        let details = account.details();

        assert_eq!(details.alias.unwrap(), "Testv20")
    }
}
