use serde::{Deserialize, Serialize};
use crate::types::constants::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseContract {
    pub security_type: SecurityType,
    pub exchange: Exchange,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    #[serde(flatten)]
    pub base: BaseContract,
    pub symbol: String,
    pub name: String,
    pub category: String,
    pub currency: Currency,
    pub delivery_month: String,
    pub delivery_date: String,
    pub strike_price: f64,
    pub option_right: OptionRight,
    pub underlying_kind: String,
    pub underlying_code: String,
    pub unit: f64,
    pub multiplier: i32,
    pub limit_up: f64,
    pub limit_down: f64,
    pub reference: f64,
    pub update_date: String,
    pub margin_trading_balance: i32,
    pub short_selling_balance: i32,
    pub day_trade: DayTrade,
    pub target_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stock {
    #[serde(flatten)]
    pub contract: Contract,
}

impl Stock {
    pub fn new(code: &str, exchange: Exchange) -> Self {
        Self {
            contract: Contract {
                base: BaseContract {
                    security_type: SecurityType::Stock,
                    exchange,
                    code: code.to_string(),
                },
                symbol: String::new(),
                name: String::new(),
                category: String::new(),
                currency: Currency::TWD,
                delivery_month: String::new(),
                delivery_date: String::new(),
                strike_price: 0.0,
                option_right: OptionRight::No,
                underlying_kind: String::new(),
                underlying_code: String::new(),
                unit: 1000.0,
                multiplier: 1,
                limit_up: 0.0,
                limit_down: 0.0,
                reference: 0.0,
                update_date: String::new(),
                margin_trading_balance: 0,
                short_selling_balance: 0,
                day_trade: DayTrade::No,
                target_code: String::new(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Future {
    #[serde(flatten)]
    pub contract: Contract,
}

impl Future {
    pub fn new(code: &str) -> Self {
        Self {
            contract: Contract {
                base: BaseContract {
                    security_type: SecurityType::Future,
                    exchange: Exchange::TAIFEX,
                    code: code.to_string(),
                },
                symbol: String::new(),
                name: String::new(),
                category: String::new(),
                currency: Currency::TWD,
                delivery_month: String::new(),
                delivery_date: String::new(),
                strike_price: 0.0,
                option_right: OptionRight::No,
                underlying_kind: String::new(),
                underlying_code: String::new(),
                unit: 1.0,
                multiplier: 200,
                limit_up: 0.0,
                limit_down: 0.0,
                reference: 0.0,
                update_date: String::new(),
                margin_trading_balance: 0,
                short_selling_balance: 0,
                day_trade: DayTrade::Yes,
                target_code: String::new(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionContract {
    #[serde(flatten)]
    pub contract: Contract,
}

impl OptionContract {
    pub fn new(code: &str, option_right: OptionRight, strike_price: f64) -> Self {
        Self {
            contract: Contract {
                base: BaseContract {
                    security_type: SecurityType::Option,
                    exchange: Exchange::TAIFEX,
                    code: code.to_string(),
                },
                symbol: String::new(),
                name: String::new(),
                category: String::new(),
                currency: Currency::TWD,
                delivery_month: String::new(),
                delivery_date: String::new(),
                strike_price,
                option_right,
                underlying_kind: String::new(),
                underlying_code: String::new(),
                unit: 1.0,
                multiplier: 50,
                limit_up: 0.0,
                limit_down: 0.0,
                reference: 0.0,
                update_date: String::new(),
                margin_trading_balance: 0,
                short_selling_balance: 0,
                day_trade: DayTrade::Yes,
                target_code: String::new(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    #[serde(flatten)]
    pub contract: Contract,
}

impl Index {
    pub fn new(code: &str, exchange: Exchange) -> Self {
        Self {
            contract: Contract {
                base: BaseContract {
                    security_type: SecurityType::Index,
                    exchange,
                    code: code.to_string(),
                },
                symbol: String::new(),
                name: String::new(),
                category: String::new(),
                currency: Currency::TWD,
                delivery_month: String::new(),
                delivery_date: String::new(),
                strike_price: 0.0,
                option_right: OptionRight::No,
                underlying_kind: String::new(),
                underlying_code: String::new(),
                unit: 1.0,
                multiplier: 1,
                limit_up: 0.0,
                limit_down: 0.0,
                reference: 0.0,
                update_date: String::new(),
                margin_trading_balance: 0,
                short_selling_balance: 0,
                day_trade: DayTrade::No,
                target_code: String::new(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComboBase {
    #[serde(flatten)]
    pub contract: Contract,
    pub action: Action,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComboContract {
    pub legs: Vec<ComboBase>,
}