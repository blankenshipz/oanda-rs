use serde_derive::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct CandlestickData {
    /// The first (open) price in the time-range represented by the candlestick.
    pub o: String,
    /// The highest price in the time-range represented by the candlestick.
    pub h: String,
    /// The lowest price in the time-range represented by the candlestick.
    pub l: String,
    /// The last (closing) price in the time-range represented by the
    /// candlestick.
    pub c: String,
}
