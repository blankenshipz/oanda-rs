use chrono::datetime::DateTime;
use chrono::UTC;

#[derive(Deserialize)]
pub enum TradeState {
    OPEN,
    CLOSED,
    CLOSE_WHEN_TRADEABLE
}

#[derive(Deserialize)]
pub enum OrderState {
    PENDING,
    FILLED,
    TRIGGERED,
    CANCELLED
}

#[derive(Deserialize)]
pub struct ClientExtensions {
    // The Client ID of the Order/Trade
    pub id: String,
    // A tag associated with the Order/Trade
    pub tag: String,
    // A comment associated with the Order/Trade
    pub comment: String
}

#[derive(Deserialize)]
pub struct TradeSummary {
    // The Trade’s identifier, unique within the Trade’s Account.
    pub id: String,
    // The Trade’s Instrument.
    pub instrument: String,
    // The execution price of the Trade.
    pub price: f32,
    // The date/time when the Trade was opened.
    pub openTime: DateTime<UTC>,
    // The current state of the Trade.
    pub state: TradeState,
    // The initial size of the Trade. Negative values indicate a short Trade,
    // and positive values indicate a long Trade.
    pub initialUnits: f32,
    // The number of units currently open for the Trade. This value is reduced
    // to 0.0 as the Trade is closed.
    pub currentUnits: f32,
    // The total profit/loss realized on the closed portion of the Trade.
    pub realizedPL: f32,
    // The unrealized profit/loss on the open portion of the Trade.
    pub unrealizedPL: f32,
    // The average closing price of the Trade. Only present if the Trade has
    // been closed or reduced at least once.
    pub averageClosePrice: Option<f32>,
    // The IDs of the Transactions that have closed portions of this Trade.
    pub closingTransactionIDs: Vec<String>,
    // The financing paid/collected for this Trade.
    pub financing: f32,
    // The date/time when the Trade was fully closed. Only provided for Trades
    // whose state is CLOSED.
    pub closeTime: Option<DateTime<UTC>>,
    // The client extensions of the Trade.
    pub clientExtensions: ClientExtensions,
    // ID of the Trade’s Take Profit Order, only provided if such an Order
    // exists.
    pub takeProfitOrderID: Option<String>,
    // ID of the Trade’s Stop Loss Order, only provided if such an Order exists.
    pub stopLossOrderID: Option<String>,
    // ID of the Trade’s Trailing Stop Loss Order, only provided if such an
    // Order exists.
    pub trailingStopLossOrderID: Option<String>
}

#[derive(Deserialize)]
pub struct PositionSide {
    // Number of units in the position (negative value indicates short position,
    // positive indicates long position).
    pub units: f32,
    // Volume-weighted average of the underlying Trade open prices for the
    // Position.
    pub averagePrice: f32,
    // List of the open Trade IDs which contribute to the open Position.
    pub tradeIDs: Vec<String>,
    // Profit/loss realized by the PositionSide over the lifetime of the
    // Account.
    pub pl: f32,
    // The unrealized profit/loss of all open Trades that contribute to this
    // PositionSide.
    pub unrealizedPL: f32,
    // Profit/loss realized by the PositionSide since the Account’s resettablePL
    // was last reset by the client.
    pub resettablePL: f32
}

#[derive(Deserialize)]
pub struct Position {
    // The Position’s Instrument.
    pub instrument: String,
    // Profit/loss realized by the Position over the lifetime of the Account.
    pub pl: f32,
    // The unrealized profit/loss of all open Trades that contribute to this
    // Position.
    pub unrealizedPL: f32,
    // Profit/loss realized by the Position since the Account’s resettablePL was
    // last reset by the client.
    pub resettablePL: f32,
    // The details of the long side of the Position.
    pub long: PositionSide,
    // The details of the short side of the Position.
    pub short: PositionSide
}

#[derive(Deserialize)]
pub struct Order {
    // The Order’s identifier, unique within the Order’s Account.
    pub id: String,
    // The time when the Order was created.
    pub createTime: DateTime<UTC>,
    // The current state of the Order.
    pub state: OrderState,
    // The client extensions of the Order. Do not set, modify, or delete
    // clientExtensions if your account is associated with MT4.
    pub clientExtensions: ClientExtensions
}

#[derive(Deserialize)]
pub struct Details {
    // The Account’s identifier
    pub id: String,
    // Client-assigned alias for the Account. Only provided if the Account has
    // an alias set
    pub alias: Option<String>,
    // The home currency of the Account
    pub currency: String,
    // The current balance of the Account. Represented in the Account’s home
    // currency.
    pub balance: f32,
    // ID of the user that created the Account.
    pub createdByUserID: i32,
    // The date/time when the Account was created.
    pub createdTime: DateTime<UTC>,
    // The total profit/loss realized over the lifetime of the Account.
    // Represented in the Account’s home currency.
    pub pl: f32,
    // The total realized profit/loss for the Account since it was last reset by
    // the client. Represented in the Account’s home currency.
    pub resettablePL: f32,
    // The date/time that the Account’s resettablePL was last reset.
    pub resettabledPLTime: Option<DateTime<UTC>>,
    // Client-provided margin rate override for the Account. The effective
    // margin rate of the Account is the lesser of this value and the OANDA
    // margin rate for the Account’s division. This value is only provided if a
    // margin rate override exists for the Account.
    pub marginRate: Option<f32>,
    // The date/time when the Account entered a margin call state. Only provided
    // if the Account is in a margin call.
    pub marginCallEnterTime: Option<DateTime<UTC>>,
    // The number of times that the Account’s current margin call was extended.
    pub marginCallExtensionCount: Option<i32>,
    // The date/time of the Account’s last margin call extension.
    pub lastMarginCallExtensionTime: Option<DateTime<UTC>>,
    // The number of Trades currently open in the Account.
    pub openTradeCount: i32,
    // The number of Positions currently open in the Account.
    pub openPositionCount: i32,
    // The number of Orders currently pending in the Account.
    pub pendingOrderCount: i32,
    // Flag indicating that the Account has hedging enabled.
    pub hedgingEnabled: bool,
    // The total unrealized profit/loss for all Trades currently open in the
    // Account. Represented in the Account’s home currency.
    pub unrealizedPL: f32,
    // The net asset value of the Account. Equal to Account balance +
    // unrealizedPL. Represented in the Account’s home currency.
    pub NAV: f32,
    // Margin currently used for the Account. Represented in the Account’s home
    // currency.
    pub marginUsed: f32,
    // Margin available for Account. Represented in the Account’s home currency.
    pub marginAvailable: f32,
    // The value of the Account’s open positions represented in the Account’s
    // home currency.
    pub positionValue: f32,
    // The Account’s margin closeout unrealized PL.
    pub marginCloseoutUnrealizedPL: f32,
    // The Account’s margin closeout NAV.
    pub marginCloseoutNAV: f32,
    // The Account’s margin closeout margin used.
    pub marginCloseoutMarginUsed: f32,
    // The Account’s margin closeout percentage. When this value is 1.0 or above
    // the Account is in a margin closeout situation.
    pub marginCloseoutPercent: f32,
    // The value of the Account’s open positions as used for margin closeout
    // calculations represented in the Account’s home currency.
    pub marginCloseoutPositionValue: f32,
    // The current WithdrawalLimit for the account which will be zero or a
    // positive value indicating how much can be withdrawn from the account.
    pub withdrawalLimit: f32,
    // The Account’s margin call margin used.
    pub marginCallMarginUsed: f32,
    // The Account’s margin call percentage. When this value is 1.0 or above the
    // Account is in a margin call situation.
    pub marginCallPercent: f32,
    // The ID of the last Transaction created for the Account.
    pub lastTransactionID: String,
    // The details of the Trades currently open in the Account.
    pub trades: Vec<TradeSummary>,
    // The details all Account Positions.
    pub positions: Vec<Position>,
    // The details of the Orders currently pending in the Account.
    pub orders: Vec<Order>
}

#[derive(Deserialize)]
pub struct AccountDetails {
    // The full details of the requested Account.
    pub account: Details,
    // The ID of the most recent Transaction created for the Account.
    lastTransactionID: String
}
