use chrono::DateTime;
use chrono::Utc;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub enum TradeState {
    OPEN,
    CLOSED,
    CLOSE_WHEN_TRADEABLE,
}

#[derive(Deserialize)]
pub enum OrderState {
    PENDING,
    FILLED,
    TRIGGERED,
    CANCELLED,
}

#[derive(Deserialize)]
pub struct ClientExtensions {
    /// The Client ID of the Order/Trade
    pub id: String,
    /// A tag associated with the Order/Trade
    pub tag: String,
    /// A comment associated with the Order/Trade
    pub comment: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeSummary {
    /// The Trade’s identifier, unique within the Trade’s Account.
    pub id: String,
    /// The Trade’s Instrument.
    pub instrument: String,
    /// The execution price of the Trade.
    pub price: f32,
    /// The date/time when the Trade was opened.
    pub open_time: DateTime<Utc>,
    /// The current state of the Trade.
    pub state: TradeState,
    /// The initial size of the Trade. Negative values indicate a short Trade,
    /// and positive values indicate a long Trade.
    pub initial_units: f32,
    /// The number of units currently open for the Trade. This value is reduced
    /// to 0.0 as the Trade is closed.
    pub current_units: f32,
    /// The total profit/loss realized on the closed portion of the Trade.
    #[serde(rename = "realizedPL")]
    pub realized_pl: f32,
    /// The unrealized profit/loss on the open portion of the Trade.
    #[serde(rename = "unrealizedPL")]
    pub unrealized_pl: f32,
    /// The average closing price of the Trade. Only present if the Trade has
    /// been closed or reduced at least once.
    pub average_close_price: Option<f32>,
    /// The IDs of the Transactions that have closed portions of this Trade.
    #[serde(rename = "closingTransactionIDs")]
    pub closing_transaction_ids: Vec<String>,
    /// The financing paid/collected for this Trade.
    pub financing: f32,
    /// The date/time when the Trade was fully closed. Only provided for Trades
    /// whose state is CLOSED.
    pub close_time: Option<DateTime<Utc>>,
    /// The client extensions of the Trade.
    pub client_extensions: ClientExtensions,
    /// ID of the Trade’s Take Profit Order, only provided if such an Order
    /// exists.
    #[serde(rename = "takeProfitOrderID")]
    pub take_profit_order_id: Option<String>,
    /// ID of the Trade’s Stop Loss Order, only provided if such an Order exists.
    #[serde(rename = "stopLossOrderID")]
    pub stop_loss_order_id: Option<String>,
    /// ID of the Trade’s Trailing Stop Loss Order, only provided if such an
    /// Order exists.
    #[serde(rename = "trailingStopLossOrderID")]
    pub trailing_stop_loss_order_id: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionSide {
    /// Number of units in the position (negative value indicates short position,
    /// positive indicates long position).
    pub units: f32,
    /// Volume-weighted average of the underlying Trade open prices for the
    /// Position.
    pub average_price: f32,
    /// List of the open Trade IDs which contribute to the open Position.
    #[serde(rename = "tradeIDs")]
    pub trade_ids: Vec<String>,
    /// Profit/loss realized by the PositionSide over the lifetime of the
    /// Account.
    pub pl: f32,
    /// The unrealized profit/loss of all open Trades that contribute to this
    /// PositionSide.
    #[serde(rename = "unrealizedPL")]
    pub unrealized_pl: f32,
    /// Profit/loss realized by the PositionSide since the Account’s resettablePL
    /// was last reset by the client.
    #[serde(rename = "resettablePL")]
    pub resettable_pl: f32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// The Position’s Instrument.
    pub instrument: String,
    /// Profit/loss realized by the Position over the lifetime of the Account.
    pub pl: f32,
    /// The unrealized profit/loss of all open Trades that contribute to this
    /// Position.
    #[serde(rename = "unrealizedPL")]
    pub unrealized_pl: f32,
    /// Profit/loss realized by the Position since the Account’s resettablePL was
    /// last reset by the client.
    #[serde(rename = "resettablePL")]
    pub resettable_pl: f32,
    /// The details of the long side of the Position.
    pub long: PositionSide,
    /// The details of the short side of the Position.
    pub short: PositionSide,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// The Order’s identifier, unique within the Order’s Account.
    pub id: String,
    /// The time when the Order was created.
    pub create_time: DateTime<Utc>,
    /// The current state of the Order.
    pub state: OrderState,
    /// The client extensions of the Order. Do not set, modify, or delete
    /// clientExtensions if your account is associated with MT4.
    pub client_extensions: ClientExtensions,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    /// The Account’s identifier
    pub id: String,
    /// Client-assigned alias for the Account. Only provided if the Account has
    /// an alias set
    pub alias: Option<String>,
    /// The home currency of the Account
    pub currency: String,
    /// The current balance of the Account. Represented in the Account’s home
    /// currency.
    pub balance: f32,
    /// ID of the user that created the Account.
    #[serde(rename = "createdByUserID")]
    pub created_by_user_id: i32,
    /// The date/time when the Account was created.
    pub created_time: DateTime<Utc>,
    /// The total profit/loss realized over the lifetime of the Account.
    /// Represented in the Account’s home currency.
    pub pl: f32,
    /// The total realized profit/loss for the Account since it was last reset by
    /// the client. Represented in the Account’s home currency.
    #[serde(rename = "resettablePL")]
    pub resettable_pl: f32,
    /// The date/time that the Account’s resettablePL was last reset.
    #[serde(rename = "resettablePLTime")]
    pub resettabled_pl_time: Option<DateTime<Utc>>,
    /// Client-provided margin rate override for the Account. The effective
    /// margin rate of the Account is the lesser of this value and the OANDA
    /// margin rate for the Account’s division. This value is only provided if a
    /// margin rate override exists for the Account.
    pub margin_rate: Option<f32>,
    /// The date/time when the Account entered a margin call state. Only provided
    /// if the Account is in a margin call.
    pub margin_call_enter_time: Option<DateTime<Utc>>,
    /// The number of times that the Account’s current margin call was extended.
    pub margin_call_extension_count: Option<i32>,
    /// The date/time of the Account’s last margin call extension.
    pub last_margin_call_extension_time: Option<DateTime<Utc>>,
    /// The number of Trades currently open in the Account.
    pub open_trade_count: i32,
    /// The number of Positions currently open in the Account.
    pub open_position_count: i32,
    /// The number of Orders currently pending in the Account.
    pub pending_order_count: i32,
    /// Flag indicating that the Account has hedging enabled.
    pub hedging_enabled: bool,
    /// The total unrealized profit/loss for all Trades currently open in the
    /// Account. Represented in the Account’s home currency.
    #[serde(rename = "unrealizedPL")]
    pub unrealized_pl: f32,
    /// The net asset value of the Account. Equal to Account balance +
    /// unrealizedPL. Represented in the Account’s home currency.
    #[serde(rename = "NAV")]
    pub nav: f32,
    /// Margin currently used for the Account. Represented in the Account’s home
    /// currency.
    pub margin_used: f32,
    /// Margin available for Account. Represented in the Account’s home currency.
    pub margin_available: f32,
    /// The value of the Account’s open positions represented in the Account’s
    /// home currency.
    pub position_value: f32,
    /// The Account’s margin closeout unrealized PL.
    #[serde(rename = "marginCloseoutUnrealizedPL")]
    pub margin_closeout_unrealized_pl: f32,
    /// The Account’s margin closeout NAV.
    #[serde(rename = "marginCloseoutNAV")]
    pub margin_closeout_nav: f32,
    /// The Account’s margin closeout margin used.
    pub margin_closeout_margin_used: f32,
    /// The Account’s margin closeout percentage. When this value is 1.0 or above
    /// the Account is in a margin closeout situation.
    pub margin_closeout_percent: f32,
    /// The value of the Account’s open positions as used for margin closeout
    /// calculations represented in the Account’s home currency.
    pub margin_closeout_position_value: f32,
    /// The current WithdrawalLimit for the account which will be zero or a
    /// positive value indicating how much can be withdrawn from the account.
    pub withdrawal_limit: f32,
    /// The Account’s margin call margin used.
    pub margin_call_margin_used: f32,
    /// The Account’s margin call percentage. When this value is 1.0 or above the
    /// Account is in a margin call situation.
    pub margin_call_percent: f32,
    /// The ID of the last Transaction created for the Account.
    #[serde(rename = "lastTransactionID")]
    pub last_transaction_id: String,
    /// The details of the Trades currently open in the Account.
    pub trades: Vec<TradeSummary>,
    /// The details all Account Positions.
    pub positions: Vec<Position>,
    /// The details of the Orders currently pending in the Account.
    pub orders: Vec<Order>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountDetails {
    /// The full details of the requested Account.
    pub account: Details,
    /// The ID of the most recent Transaction created for the Account.
    #[serde(rename = "lastTransactionID")]
    pub last_transaction_id: String,
}
