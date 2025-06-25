use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::types::constants::*;
use crate::types::contracts::Contract;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickSTKv1 {
    pub code: String,
    pub datetime: DateTime<Utc>,
    pub open: f64,
    pub avg_price: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub amount: f64,
    pub total_amount: f64,
    pub volume: i64,
    pub total_volume: i64,
    pub tick_type: TickType,
    pub chg_type: ChangeType,
    pub price_chg: f64,
    pub pct_chg: f64,
    pub bid_side_total_vol: i64,
    pub ask_side_total_vol: i64,
    pub bid_side_total_cnt: i64,
    pub ask_side_total_cnt: i64,
    pub suspend: bool,
    pub simtrade: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickFOPv1 {
    pub code: String,
    pub datetime: DateTime<Utc>,
    pub open: f64,
    pub underlying_price: f64,
    pub bid_side_total_vol: i64,
    pub ask_side_total_vol: i64,
    pub avg_price: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub amount: f64,
    pub total_amount: f64,
    pub volume: i64,
    pub total_volume: i64,
    pub tick_type: TickType,
    pub chg_type: ChangeType,
    pub price_chg: f64,
    pub pct_chg: f64,
    pub simtrade: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidAskSTKv1 {
    pub code: String,
    pub datetime: DateTime<Utc>,
    pub bid_price: Vec<f64>,
    pub bid_volume: Vec<i64>,
    pub diff_bid_vol: Vec<i64>,
    pub ask_price: Vec<f64>,
    pub ask_volume: Vec<i64>,
    pub diff_ask_vol: Vec<i64>,
    pub suspend: bool,
    pub simtrade: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidAskFOPv1 {
    pub code: String,
    pub datetime: DateTime<Utc>,
    pub bid_price: Vec<f64>,
    pub bid_volume: Vec<i64>,
    pub diff_bid_vol: Vec<i64>,
    pub ask_price: Vec<f64>,
    pub ask_volume: Vec<i64>,
    pub diff_ask_vol: Vec<i64>,
    pub underlying_price: f64,
    pub simtrade: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteSTKv1 {
    pub code: String,
    pub datetime: DateTime<Utc>,
    pub bid_price: f64,
    pub bid_volume: i64,
    pub ask_price: f64,
    pub ask_volume: i64,
    pub close: f64,
    pub volume: i64,
    pub amount: f64,
    pub suspend: bool,
    pub simtrade: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kbar {
    pub ts: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kbars {
    pub contract: Contract,
    pub data: Vec<Kbar>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tick {
    pub ts: DateTime<Utc>,
    pub close: f64,
    pub volume: i64,
    pub bid_price: f64,
    pub bid_volume: i64,
    pub ask_price: f64,
    pub ask_volume: i64,
    pub tick_type: TickType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticks {
    pub contract: Contract,
    pub data: Vec<Tick>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub contract: Contract,
    pub ts: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
    pub amount: f64,
    pub bid_price: f64,
    pub bid_volume: i64,
    pub ask_price: f64,
    pub ask_volume: i64,
    pub total_amount: f64,
    pub total_volume: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyQuote {
    pub contract: Contract,
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
    pub amount: f64,
    pub price_change: f64,
    pub percent_change: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyQuotes {
    pub date: String,
    pub data: Vec<DailyQuote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerItem {
    pub contract: Contract,
    pub value: f64,
    pub rank: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScannerType {
    ChangePercentRank,
    ChangePriceRank,
    DayRangeRank,
    VolumeRank,
    AmountRank,
    TickCountRank,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditEnquire {
    pub contract: Contract,
    pub margin_trading_balance: i64,
    pub margin_trading_limit: i64,
    pub short_selling_balance: i64,
    pub short_selling_limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortStockSource {
    pub contract: Contract,
    pub available_volume: i64,
    pub fee_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStatus {
    pub api_call_count: i32,
    pub api_call_limit: i32,
    pub rate_limit_remaining: i32,
    pub rate_limit_reset: DateTime<Utc>,
}

// Simple Default implementations for callback data types
impl Default for TickSTKv1 {
    fn default() -> Self {
        Self {
            code: String::new(),
            datetime: Utc::now(),
            open: 0.0,
            avg_price: 0.0,
            close: 0.0,
            high: 0.0,
            low: 0.0,
            amount: 0.0,
            total_amount: 0.0,
            volume: 0,
            total_volume: 0,
            tick_type: TickType::No,
            chg_type: ChangeType::Unchanged,
            price_chg: 0.0,
            pct_chg: 0.0,
            bid_side_total_vol: 0,
            ask_side_total_vol: 0,
            bid_side_total_cnt: 0,
            ask_side_total_cnt: 0,
            suspend: false,
            simtrade: false,
        }
    }
}

impl Default for TickFOPv1 {
    fn default() -> Self {
        Self {
            code: String::new(),
            datetime: Utc::now(),
            open: 0.0,
            underlying_price: 0.0,
            bid_side_total_vol: 0,
            ask_side_total_vol: 0,
            avg_price: 0.0,
            close: 0.0,
            high: 0.0,
            low: 0.0,
            amount: 0.0,
            total_amount: 0.0,
            volume: 0,
            total_volume: 0,
            tick_type: TickType::No,
            chg_type: ChangeType::Unchanged,
            price_chg: 0.0,
            pct_chg: 0.0,
            simtrade: false,
        }
    }
}

impl Default for BidAskSTKv1 {
    fn default() -> Self {
        Self {
            code: String::new(),
            datetime: Utc::now(),
            bid_price: vec![0.0; 5],
            bid_volume: vec![0; 5],
            diff_bid_vol: vec![0; 5],
            ask_price: vec![0.0; 5],
            ask_volume: vec![0; 5],
            diff_ask_vol: vec![0; 5],
            suspend: false,
            simtrade: false,
        }
    }
}

impl Default for BidAskFOPv1 {
    fn default() -> Self {
        Self {
            code: String::new(),
            datetime: Utc::now(),
            underlying_price: 0.0,
            bid_price: vec![0.0; 5],
            bid_volume: vec![0; 5],
            diff_bid_vol: vec![0; 5],
            ask_price: vec![0.0; 5],
            ask_volume: vec![0; 5],
            diff_ask_vol: vec![0; 5],
            simtrade: false,
        }
    }
}

impl Default for QuoteSTKv1 {
    fn default() -> Self {
        Self {
            code: String::new(),
            datetime: Utc::now(),
            bid_price: 0.0,
            bid_volume: 0,
            ask_price: 0.0,
            ask_volume: 0,
            close: 0.0,
            volume: 0,
            amount: 0.0,
            suspend: false,
            simtrade: false,
        }
    }
}