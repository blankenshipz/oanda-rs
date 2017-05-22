use chrono::datetime::DateTime;
use chrono::UTC;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
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
    #[serde(rename = "createdByUserID")]
    pub created_by_user_id: i32,
    // The date/time when the Account was created.
    pub created_time: DateTime<UTC>,
    // The total profit/loss realized over the lifetime of the Account.
    // Represented in the Account’s home currency.
    pub pl: f32,
    // The total realized profit/loss for the Account since it was last reset by
    // the client. Represented in the Account’s home currency.
    #[serde(rename = "resettablePL")]
    pub resettable_pL: f32,
    // The date/time that the Account’s resettablePL was last reset.
    #[serde(rename = "resettabledPLTimelast")]
    pub resettabled_pl_time: Option<DateTime<UTC>>,
    // Client-provided margin rate override for the Account. The effective
    // margin rate of the Account is the lesser of this value and the OANDA
    // margin rate for the Account’s division. This value is only provided if a
    // margin rate override exists for the Account.
    pub margin_rate: Option<f32>,
    // The date/time when the Account entered a margin call state. Only provided
    // if the Account is in a margin call.
    pub margin_call_enter_time: Option<DateTime<UTC>>,
    // The number of times that the Account’s current margin call was extended.
    pub margin_call_extension_count: Option<i32>,
    // The date/time of the Account’s last margin call extension.
    pub last_margin_call_extension_time: Option<DateTime<UTC>>,
    // The number of Trades currently open in the Account.
    pub open_trade_count: i32,
    // The number of Positions currently open in the Account.
    pub open_position_count: i32,
    // The number of Orders currently pending in the Account.
    pub pending_order_count: i32,
    // Flag indicating that the Account has hedging enabled.
    pub hedging_enabled: bool,
    // The total unrealized profit/loss for all Trades currently open in the
    // Account. Represented in the Account’s home currency.
    #[serde(rename = "unrealizedPL")]
    pub unrealized_pl: f32,
    // The net asset value of the Account. Equal to Account balance +
    // unrealizedPL. Represented in the Account’s home currency.
    #[serde(rename = "NAV")]
    pub nav: f32,
    // Margin currently used for the Account. Represented in the Account’s home
    // currency.
    pub margin_used: f32,
    // Margin available for Account. Represented in the Account’s home currency.
    pub margin_available: f32,
    // The value of the Account’s open positions represented in the Account’s
    // home currency.
    pub position_value: f32,
    // The Account’s margin closeout unrealized PL.
    #[serde(rename = "marginCloseoutUnrealizedPL")]
    pub margin_closeout_unrealized_pl: f32,
    // The Account’s margin closeout NAV.
    #[serde(rename = "marginCloseoutNAV")]
    pub margin_closeout_nav: f32,
    // The Account’s margin closeout margin used.
    pub margin_closeout_margin_used: f32,
    // The Account’s margin closeout percentage. When this value is 1.0 or above
    // the Account is in a margin closeout situation.
    pub margin_closeout_percent: f32,
    // The value of the Account’s open positions as used for margin closeout
    // calculations represented in the Account’s home currency.
    pub margin_closeout_position_value: f32,
    // The current WithdrawalLimit for the account which will be zero or a
    // positive value indicating how much can be withdrawn from the account.
    pub withdrawal_limit: f32,
    // The Account’s margin call margin used.
    pub margin_call_margin_used: f32,
    // The Account’s margin call percentage. When this value is 1.0 or above the
    // Account is in a margin call situation.
    pub margin_call_percent: f32,
    // The ID of the last Transaction created for the Account.
    #[serde(rename = "lastTransactionID")]
    pub last_transaction_id: String
}

#[derive(Deserialize)]
pub struct AccountSummary {
    // The full details of the requested Account.
    pub account: Summary,
    // The ID of the most recent Transaction created for the Account.
    #[serde(rename = "lastTransactionID")]
    pub last_transaction_id: String
}
