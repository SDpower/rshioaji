use serde::{Deserialize, Serialize};
use crate::types::constants::*;
use crate::types::accounts::Account;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub action: Action,
    pub price: f64,
    pub quantity: i32,
    pub order_type: OrderType,
    pub price_type: StockPriceType,
    pub order_lot: Option<StockOrderLot>,
    pub order_cond: Option<StockOrderCond>,
    pub first_sell: Option<bool>,
    pub account: Option<Account>,
    pub ca: Option<String>,
    pub seqno: Option<String>,
}

impl Order {
    pub fn new(
        action: Action,
        price: f64,
        quantity: i32,
        order_type: OrderType,
        price_type: StockPriceType,
    ) -> Self {
        Self {
            action,
            price,
            quantity,
            order_type,
            price_type,
            order_lot: None,
            order_cond: None,
            first_sell: None,
            account: None,
            ca: None,
            seqno: None,
        }
    }
    
    pub fn with_account(mut self, account: Account) -> Self {
        self.account = Some(account);
        self
    }
    
    pub fn with_order_lot(mut self, order_lot: StockOrderLot) -> Self {
        self.order_lot = Some(order_lot);
        self
    }
    
    pub fn with_order_cond(mut self, order_cond: StockOrderCond) -> Self {
        self.order_cond = Some(order_cond);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesOrder {
    pub action: Action,
    pub price: f64,
    pub quantity: i32,
    pub order_type: OrderType,
    pub price_type: FuturesPriceType,
    pub octype: FuturesOCType,
    pub account: Option<Account>,
    pub ca: Option<String>,
    pub seqno: Option<String>,
}

impl FuturesOrder {
    pub fn new(
        action: Action,
        price: f64,
        quantity: i32,
        order_type: OrderType,
        price_type: FuturesPriceType,
        octype: FuturesOCType,
    ) -> Self {
        Self {
            action,
            price,
            quantity,
            order_type,
            price_type,
            octype,
            account: None,
            ca: None,
            seqno: None,
        }
    }
    
    pub fn with_account(mut self, account: Account) -> Self {
        self.account = Some(account);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComboOrder {
    pub price: f64,
    pub quantity: i32,
    pub order_type: OrderType,
    pub account: Option<Account>,
    pub ca: Option<String>,
    pub seqno: Option<String>,
}

impl ComboOrder {
    pub fn new(price: f64, quantity: i32, order_type: OrderType) -> Self {
        Self {
            price,
            quantity,
            order_type,
            account: None,
            ca: None,
            seqno: None,
        }
    }
    
    pub fn with_account(mut self, account: Account) -> Self {
        self.account = Some(account);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub order: Order,
    pub status: Status,
    pub order_id: String,
    pub seqno: String,
    pub ordno: String,
    pub account: Account,
    pub contracts: Vec<crate::types::contracts::Contract>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesTrade {
    pub order: FuturesOrder,
    pub status: Status,
    pub order_id: String,
    pub seqno: String,
    pub ordno: String,
    pub account: Account,
    pub contracts: Vec<crate::types::contracts::Contract>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComboTrade {
    pub order: ComboOrder,
    pub status: Status,
    pub order_id: String,
    pub seqno: String,
    pub ordno: String,
    pub account: Account,
    pub combo_contract: crate::types::contracts::ComboContract,
}

/// Order state for callbacks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderState {
    /// Order has been submitted
    Submitted,
    /// Order is filled
    Filled,
    /// Order is partially filled
    PartFilled,
    /// Order is cancelled
    Cancelled,
    /// Order failed
    Failed,
    /// Order is pending submit
    PendingSubmit,
    /// Order is pending cancel
    PendingCancel,
    /// Order is waiting for manual approval
    WaitingManualApproval,
    /// Order is rejected
    Rejected,
}

impl std::fmt::Display for OrderState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderState::Submitted => write!(f, "Submitted"),
            OrderState::Filled => write!(f, "Filled"),
            OrderState::PartFilled => write!(f, "PartFilled"),
            OrderState::Cancelled => write!(f, "Cancelled"),
            OrderState::Failed => write!(f, "Failed"),
            OrderState::PendingSubmit => write!(f, "PendingSubmit"),
            OrderState::PendingCancel => write!(f, "PendingCancel"),
            OrderState::WaitingManualApproval => write!(f, "WaitingManualApproval"),
            OrderState::Rejected => write!(f, "Rejected"),
        }
    }
}

impl From<&str> for OrderState {
    fn from(s: &str) -> Self {
        match s {
            "Submitted" => OrderState::Submitted,
            "Filled" => OrderState::Filled,
            "PartFilled" => OrderState::PartFilled,
            "Cancelled" => OrderState::Cancelled,
            "Failed" => OrderState::Failed,
            "PendingSubmit" => OrderState::PendingSubmit,
            "PendingCancel" => OrderState::PendingCancel,
            "WaitingManualApproval" => OrderState::WaitingManualApproval,
            "Rejected" => OrderState::Rejected,
            _ => OrderState::Failed,
        }
    }
}