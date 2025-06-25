use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccountType {
    Stock,      // S - 股票帳戶
    Future,     // F - 期貨帳戶
    Simulation, // H - 模擬帳戶
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Stock => write!(f, "S"),
            AccountType::Future => write!(f, "F"),
            AccountType::Simulation => write!(f, "H"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub broker_id: String,
    pub account_id: String,
    pub account_type: AccountType,
    pub username: String,
    pub signed: bool,
    pub person_id: Option<String>,
}

impl Account {
    pub fn new(
        broker_id: String,
        account_id: String,
        account_type: AccountType,
        username: String,
        signed: bool,
    ) -> Self {
        Self {
            broker_id,
            account_id,
            account_type,
            username,
            signed,
            person_id: None,
        }
    }
    
    pub fn with_person_id(mut self, person_id: String) -> Self {
        self.person_id = Some(person_id);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockAccount {
    #[serde(flatten)]
    pub account: Account,
    pub funds: Option<f64>,
    pub available_funds: Option<f64>,
    pub margin_limit: Option<f64>,
    pub short_limit: Option<f64>,
}

impl StockAccount {
    pub fn new(account: Account) -> Self {
        Self {
            account,
            funds: None,
            available_funds: None,
            margin_limit: None,
            short_limit: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutureAccount {
    #[serde(flatten)]
    pub account: Account,
    pub margin: Option<f64>,
    pub available_margin: Option<f64>,
    pub unrealized_pnl: Option<f64>,
    pub realized_pnl: Option<f64>,
}

impl FutureAccount {
    pub fn new(account: Account) -> Self {
        Self {
            account,
            margin: None,
            available_margin: None,
            unrealized_pnl: None,
            realized_pnl: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    pub account_type: AccountType,
    pub balance: f64,
    pub available_balance: f64,
    pub buying_power: f64,
    pub currency: crate::types::constants::Currency,
}