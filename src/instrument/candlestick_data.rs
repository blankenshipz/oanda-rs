#[derive(Deserialize)]
pub struct CandlestickData {
    // The first (open) price in the time-range represented by the candlestick.
    o: f32,
    // The highest price in the time-range represented by the candlestick.
    h: f32,
    // The lowest price in the time-range represented by the candlestick.
    l: f32,
    // The last (closing) price in the time-range represented by the
    // candlestick.
    c: f32
}

