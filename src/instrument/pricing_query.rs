use std::error::Error;
use std::fmt;

use serde_json;

use chrono::DateTime;
use chrono::Utc;

use super::pricing::Pricing;
use crate::client::Client;

pub struct PricingQuery<'a> {
    /// Name of the Instrument [required]
    instrument: String,
    /// The Price component(s) to get candlestick data for. Can contain any
    /// combination of the characters “M” (midpoint candles) “B” (bid candles)
    /// and “A” (ask candles). [default=M]
    price: Option<String>,
    /// The granularity of the candlesticks to fetch [default=S5]
    granularity: Option<String>,
    /// The number of candlesticks to return in the reponse. Count should not
    /// be specified if both the start and end parameters are provided, as the
    /// time range combined with the graularity will determine the number of
    /// candlesticks to return. [default=500, maximum=5000]
    count: Option<i32>,
    /// The start of the time range to fetch candlesticks for.
    from: DateTime<Utc>,
    /// The end of the time range to fetch candlesticks for.
    to: Option<DateTime<Utc>>,
    /// A flag that controls whether the candlestick is “smoothed” or not.
    /// A smoothed candlestick uses the previous candle’s close price as its
    /// open price, while an unsmoothed candlestick uses the first price from
    /// its time range as its open price. [default=False]
    smooth: Option<bool>,
    /// A flag that controls whether the candlestick that is covered by the from
    /// time should be included in the results. This flag enables clients to use
    /// the timestamp of the last completed candlestick received to poll for
    /// future candlesticks but avoid receiving the previous candlestick
    /// repeatedly. [default=True]
    include_first: Option<bool>,
    /// The hour of the day (in the specified timezone) to use for granularities
    /// that have daily alignments. [default=17, minimum=0, maximum=23]
    daily_alignment: Option<i32>,
    /// The timezone to use for the dailyAlignment parameter. Candlesticks with
    /// daily alignment will be aligned to the dailyAlignment hour within the
    /// alignmentTimezone. [default=America/New_York]
    alignment_timezone: Option<String>,
    /// The day of the week used for granularities that have weekly alignment.
    /// [default=Friday]
    weekly_alignment: Option<String>,
    /// the client
    client: &'a Client<'a>,
}

impl<'a> fmt::Display for PricingQuery<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = format!("{}/candles", self.instrument);
        let add_result =
            |s: &str, display: &str, mem: &mut String| mem.push_str(&format!("&{}={}", display, s));

        // we should always have from
        result.push_str(&format!("?from={}", self.from.to_rfc3339()));
        // we may or may not have these 'optional' attributes
        if let Some(ref price) = self.price {
            add_result(price, "price", &mut result)
        }
        if let Some(ref granularity) = self.granularity {
            add_result(granularity, "granularity", &mut result)
        }
        if let Some(ref count) = self.count {
            add_result(&count.to_string(), "count", &mut result)
        }
        if let Some(ref to) = self.to {
            add_result(&to.to_string(), "to", &mut result)
        }
        if let Some(ref smooth) = self.smooth {
            add_result(&smooth.to_string(), "smooth", &mut result)
        }
        if let Some(ref include_first) = self.include_first {
            add_result(&include_first.to_string(), "includeFirst", &mut result)
        }
        if let Some(ref daily_alignment) = self.daily_alignment {
            add_result(&daily_alignment.to_string(), "dailyAlignment", &mut result)
        }
        if let Some(ref alignment_timezone) = self.alignment_timezone {
            add_result(
                &alignment_timezone.to_string(),
                "alignmentTimezone",
                &mut result,
            )
        }
        if let Some(ref weekly_alignment) = self.weekly_alignment {
            add_result(
                &weekly_alignment.to_string(),
                "weeklyAlignment",
                &mut result,
            )
        }

        write!(f, "{}", result)
    }
}

impl<'a> PricingQuery<'a> {
    pub fn new(client: &'a Client, instrument: String, from: DateTime<Utc>) -> PricingQuery<'a> {
        PricingQuery {
            instrument,
            price: None,
            granularity: None,
            count: None,
            from,
            to: None,
            smooth: None,
            include_first: None,
            daily_alignment: None,
            alignment_timezone: None,
            weekly_alignment: None,
            client,
        }
    }
    pub fn with_price(&mut self, price: String) -> &mut PricingQuery<'a> {
        self.price = Some(price);
        self
    }

    pub fn with_granularity(&mut self, granularity: String) -> &mut PricingQuery<'a> {
        self.granularity = Some(granularity);
        self
    }

    pub fn with_count(&mut self, count: i32) -> &mut PricingQuery<'a> {
        self.count = Some(count);
        self
    }

    pub fn with_to(&mut self, to: DateTime<Utc>) -> &mut PricingQuery<'a> {
        self.to = Some(to);
        self
    }

    pub fn with_smooth(&mut self, smooth: bool) -> &mut PricingQuery<'a> {
        self.smooth = Some(smooth);
        self
    }

    pub fn with_include_first(&mut self, include_first: bool) -> &mut PricingQuery<'a> {
        self.include_first = Some(include_first);
        self
    }

    pub fn with_daily_alignment(&mut self, daily_alignment: i32) -> &mut PricingQuery<'a> {
        self.daily_alignment = Some(daily_alignment);
        self
    }

    pub fn with_alignment_timezone(&mut self, alignment_timezone: String) -> &mut PricingQuery<'a> {
        self.alignment_timezone = Some(alignment_timezone);
        self
    }

    pub fn with_weekly_alignment(&mut self, weekly_alignment: String) -> &mut PricingQuery<'a> {
        self.weekly_alignment = Some(weekly_alignment);
        self
    }

    pub async fn execute(&self) -> Result<Pricing, Box<dyn Error>> {
        let input = self
            .client
            .get(&format!("instruments/{}", self))
            .await
            .map_err(|e| {
                eprintln!("Error executing query: {}", e);
                e
            })?;
        let result: Pricing = serde_json::from_str(&input)
            .map_err(|e| {
                eprintln!("Error parsing response: {}", e);
                eprintln!("Body: {input}");
                e
            })?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use std::env;

    #[tokio::test]
    async fn it_can_perform_a_query() {
        let utc = Utc.with_ymd_and_hms(2017, 6, 21, 12, 0, 0).unwrap();
        let url = env::var("OANDA_API_URL").unwrap();
        let key = env::var("OANDA_API_KEY").unwrap();
        let account_id = env::var("OANDA_TEST_ACCOUNT_ID").unwrap();
        let client = Client::new(&url, &key);
        let mut iq = PricingQuery::new(&client, "EUR_USD".to_string(), utc);
        let query = iq.with_price("M".to_string());

        assert_eq!(
            query.to_string(),
            format!("EUR_USD/candles?from={}&price=M", utc.to_rfc3339())
        );

        assert_eq!(query.execute().await.unwrap().instrument, "EUR_USD")
    }
}
