use super::candlestick_granularity::CandlestickGranularity;
use super::candlestick::Candlestick;

#[derive(Deserialize)]
pub struct Pricing {
    // The instrument whose Prices are represented by the candlesticks.
    pub instrument: String,
    // The granularity of the candlesticks provided.
    pub granularity: CandlestickGranularity,
    // The list of candlesticks that satisfy the request.
    pub candles: Vec<Candlestick>
}
