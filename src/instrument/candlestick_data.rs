use serde_derive::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct CandlestickData {
    /// The first (open) price in the time-range represented by the candlestick.
    pub o: f32,
    /// The highest price in the time-range represented by the candlestick.
    pub h: f32,
    /// The lowest price in the time-range represented by the candlestick.
    pub l: f32,
    /// The last (closing) price in the time-range represented by the
    /// candlestick.
    pub c: f32,
}
