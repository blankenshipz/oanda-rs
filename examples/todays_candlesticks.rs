extern crate chrono;

use chrono::prelude::*;

extern crate oandars;

use oandars::client::Client;
use std::env;

#[tokio::main]
async fn main() {
    // Eastern Daylight Time - -04:00
    let est = FixedOffset::west_opt(4 * 60 * 60).unwrap();
    let start_of_day = NaiveTime::from_hms_opt(8, 0, 0).unwrap();
    let start_of_day = Utc::now().date_naive().and_time(start_of_day);
    let start_of_day = est
        .from_local_datetime(&start_of_day)
        .unwrap()
        .with_timezone(&Utc);
    // Pull in the url for OANDA API and the API Key
    // e.g https://api-fxpractice.oanda.com/v3
    let url = env::var("OANDA_API_URL").expect("OANDA_API_URL must be set");
    let key = env::var("OANDA_API_KEY").expect("OANDA_API_KEY must be set");
    // Create a new client
    let client = Client::new(&url, &key);

    // Get the first set of candles for today for this instrument
    let mut results = client
        .pricing_for("EUR_USD".to_string(), start_of_day)
        .with_include_first(false)
        .execute()
        .await
        .unwrap();

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
        results = client
            .pricing_for("EUR_USD".to_string(), results.candles.last().unwrap().time)
            .with_include_first(false)
            .execute()
            .await
            .unwrap();
    }
}
