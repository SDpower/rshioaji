use serde::{Deserialize, Serialize};
use crate::types::constants::*;
use crate::types::contracts::Contract;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub contract: Contract,
    pub quantity: i32,
    pub average_price: f64,
    pub current_price: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    pub market_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPosition {
    #[serde(flatten)]
    pub position: Position,
    pub available_quantity: i32,
    pub margin_trading_quantity: i32,
    pub short_selling_quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturePosition {
    #[serde(flatten)]
    pub position: Position,
    pub direction: Action, // Long or Short
    pub margin_required: f64,
    pub maintenance_margin: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPositionDetail {
    pub contract: Contract,
    pub buy_quantity: i32,
    pub sell_quantity: i32,
    pub buy_amount: f64,
    pub sell_amount: f64,
    pub average_buy_price: f64,
    pub average_sell_price: f64,
    pub net_quantity: i32,
    pub net_amount: f64,
    pub realized_pnl: f64,
    pub unrealized_pnl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturePositionDetail {
    pub contract: Contract,
    pub direction: Action,
    pub open_quantity: i32,
    pub close_quantity: i32,
    pub net_quantity: i32,
    pub average_open_price: f64,
    pub average_close_price: f64,
    pub realized_pnl: f64,
    pub unrealized_pnl: f64,
    pub margin_required: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockProfitLoss {
    pub contract: Contract,
    pub buy_quantity: i32,
    pub sell_quantity: i32,
    pub buy_amount: f64,
    pub sell_amount: f64,
    pub realized_pnl: f64,
    pub fee: f64,
    pub tax: f64,
    pub net_pnl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutureProfitLoss {
    pub contract: Contract,
    pub open_quantity: i32,
    pub close_quantity: i32,
    pub realized_pnl: f64,
    pub fee: f64,
    pub net_pnl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockProfitDetail {
    pub trade_date: String,
    pub contract: Contract,
    pub action: Action,
    pub quantity: i32,
    pub price: f64,
    pub amount: f64,
    pub fee: f64,
    pub tax: f64,
    pub net_amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutureProfitDetail {
    pub trade_date: String,
    pub contract: Contract,
    pub action: Action,
    pub quantity: i32,
    pub price: f64,
    pub pnl: f64,
    pub fee: f64,
    pub net_pnl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockProfitLossSummary {
    pub total_buy_amount: f64,
    pub total_sell_amount: f64,
    pub total_realized_pnl: f64,
    pub total_fee: f64,
    pub total_tax: f64,
    pub net_profit_loss: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutureProfitLossSummary {
    pub total_realized_pnl: f64,
    pub total_fee: f64,
    pub net_profit_loss: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Margin {
    pub account_balance: f64,
    pub available_margin: f64,
    pub initial_margin: f64,
    pub maintenance_margin: f64,
    pub margin_call: f64,
    pub unrealized_pnl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settlement {
    pub date: String,
    pub amount: f64,
    pub currency: Currency,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementV1 {
    pub date: String,
    pub t_money: f64,
    pub t1_money: f64,
    pub t2_money: f64,
}