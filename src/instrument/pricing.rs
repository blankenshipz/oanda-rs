use super::candlestick_granularity::CandlestickGranularity;
use super::candlestick::Candlestick;

#[derive(Deserialize)]
pub struct pricing {
    // The instrument whose Prices are represented by the candlesticks.
    instrument: String,
    // The granularity of the candlesticks provided.
    granularity: CandlestickGranularity,
    // The list of candlesticks that satisfy the request.
    candles: Vec<Candlestick>
}
