extern crate chrono;

use chrono::prelude::*;

extern crate oandars;

use std::env;
use oandars::client::Client;

fn main() {
    // Eastern Daylight Time - -04:00
    let est = chrono::FixedOffset::west(4 * 60 * 60);
    let start_of_day = chrono::NaiveTime::from_hms(8, 0, 0);
    let start_of_day = UTC::today().naive_utc().and_time(start_of_day);
    let start_of_day = est.from_local_datetime(&start_of_day)
        .unwrap()
        .with_timezone(&UTC);
    // Pull in the url for OANDA API and the API Key
    // e.g https://api-fxpractice.oanda.com/v3
    let url = env::var("OANDA_API_URL").unwrap();
    let key = env::var("OANDA_API_KEY").unwrap();
    // create a new client
    let client = Client::new(&url, &key);

    // Get the first set of candles for today for this instrument
    let mut results = client.pricing_for("EUR_USD".to_string(), start_of_day)
        .with_include_first(false)
        .execute();

    while results.candles.len() > 0 {
        // For Each Candle in our new Set Print the Open price and the Timestamp
        for candle in &results.candles {
            println!("Timestamp: {}", candle.time);
            println!("-------------------------------------------------------");
            println!(" Open: {}", candle.mid.as_ref().unwrap().o);
            println!(" High: {}", candle.mid.as_ref().unwrap().h);
            println!("  Low: {}", candle.mid.as_ref().unwrap().l);
            println!("Close: {}", candle.mid.as_ref().unwrap().c);
            println!("#######################################################");
        }

        // Load the next set of Candles
        results = client.pricing_for("EUR_USD".to_string(), results.candles.last().unwrap().time)
            .with_include_first(false)
            .execute();
    }
}
