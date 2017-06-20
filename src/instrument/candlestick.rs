use chrono::datetime::DateTime;
use chrono::UTC;

use super::candlestick_granularity::CandlestickGranularity;
use super::candlestick_data::CandlestickData;

#[derive(Deserialize)]
pub struct Candlestick {
    // The start time of the candlestick
    time: DateTime<UTC>,
    // The candlestick data based on bids. Only provided if bid-based candles
    // were requested.
    bid: CandlestickData,
    // The candlestick data based on asks. Only provided if ask-based candles
    // were requested.
    ask: CandlestickData,
    // The candlestick data based on midpoints. Only provided if midpoint-based
    // candles were requested.
    mid: CandlestickData,
    // The number of prices created during the time-range represented by the
    // candlestick.
    volume: i32,
    // A flag indicating if the candlestick is complete. A complete candlestick
    // is one whose ending time is not in the future.
    complete: bool
}
