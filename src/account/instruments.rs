use serde_derive::Deserialize;

#[derive(Deserialize)]
pub enum InstrumentType {
    /// Currency
    CURRENCY,
    ///Contract For Difference
    CFD,
    /// METAL
    METAL,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    /// The name of the Instrument
    pub name: String,
    /// The type of the Instrument
    #[serde(rename = "type")]
    pub instrument_type: InstrumentType,
    /// The display name of the Instrument
    pub display_name: String,
    /// The location of the “pip” for this instrument. The decimal position of
    /// the pip in this Instrument’s price can be found at 10 ^ pipLocation e.g.
    /// -4 pipLocation results in a decimal pip position of 10 ^ -4 = 0.0001.
    pub pip_location: i32,
    /// The number of decimal places that should be used to display prices for
    /// this instrument. e.g. a displayPrecision of 5 would result in a price of
    /// “1” being displayed as “1.00000”
    pub display_precision: i32,
    /// The amount of decimal places that may be provided when specifying the
    /// number of units traded for this instrument.
    pub trade_units_precision: i32,
    /// The smallest number of units allowed to be traded for this instrument.
    pub minimum_trade_size: f32,
    /// The maximum trailing stop distance allowed for a trailing stop loss
    /// created for this instrument. Specified in price units.
    pub maximum_trailing_stop_distance: f32,
    /// The minimum trailing stop distance allowed for a trailing stop loss
    /// created for this instrument. Specified in price units.
    pub minimum_trailing_stop_distance: f32,
    /// The maximum position size allowed for this instrument. Specified in
    /// units.
    pub maximum_position_size: f32,
    /// The maximum units allowed for an Order placed for this instrument.
    /// Specified in units.
    pub maximum_order_units: f32,
    /// The margin rate for this instrument.
    pub margin_rate: f32,
}

#[derive(Deserialize)]
pub struct AccountInstruments {
    /// The requested list of instruments.
    pub instruments: Vec<Instrument>,
    /// The ID of the most recent Transaction created for the Account.
    #[serde(rename = "lastTransactionID")]
    pub last_transaction_id: String,
}
