use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    Buy,
    Sell,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Buy => write!(f, "Buy"),
            Action::Sell => write!(f, "Sell"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Exchange {
    TSE,    // 台灣證券交易所
    OTC,    // 櫃買中心
    OES,    // 興櫃
    TAIFEX, // 期交所
}

impl std::fmt::Display for Exchange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exchange::TSE => write!(f, "TSE"),
            Exchange::OTC => write!(f, "OTC"),
            Exchange::OES => write!(f, "OES"),
            Exchange::TAIFEX => write!(f, "TAIFEX"),
        }
    }
}

impl std::str::FromStr for Exchange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TSE" => Ok(Exchange::TSE),
            "OTC" => Ok(Exchange::OTC),
            "OES" => Ok(Exchange::OES),
            "TAIFEX" => Ok(Exchange::TAIFEX),
            _ => Err(format!("Unknown exchange: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecurityType {
    Index,  // 指數
    Stock,  // 股票
    Future, // 期貨
    Option, // 選擇權
}

impl std::fmt::Display for SecurityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityType::Index => write!(f, "IND"),
            SecurityType::Stock => write!(f, "STK"),
            SecurityType::Future => write!(f, "FUT"),
            SecurityType::Option => write!(f, "OPT"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderType {
    ROD, // 當日有效
    IOC, // 立即成交否則取消
    FOK, // 全部成交否則取消
}

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderType::ROD => write!(f, "ROD"),
            OrderType::IOC => write!(f, "IOC"),
            OrderType::FOK => write!(f, "FOK"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StockPriceType {
    LMT, // 限價
    MKT, // 市價
}

impl std::fmt::Display for StockPriceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StockPriceType::LMT => write!(f, "LMT"),
            StockPriceType::MKT => write!(f, "MKT"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FuturesPriceType {
    LMT, // 限價
    MKT, // 市價
    MKP, // 市價範圍
}

impl std::fmt::Display for FuturesPriceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FuturesPriceType::LMT => write!(f, "LMT"),
            FuturesPriceType::MKT => write!(f, "MKT"),
            FuturesPriceType::MKP => write!(f, "MKP"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StockOrderLot {
    Common,      // 整股
    BlockTrade,  // 鉅額
    Fixing,      // 定盤
    Odd,         // 零股
    IntradayOdd, // 盤中零股
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StockOrderCond {
    Cash,          // 現股
    MarginTrading, // 融資
    ShortSelling,  // 融券
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FuturesOCType {
    Auto,     // 自動
    New,      // 新倉
    Cover,    // 平倉
    DayTrade, // 當沖
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptionRight {
    No,   // 無
    Call, // 買權
    Put,  // 賣權
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Status {
    Cancelled,     // 已取消
    Filled,        // 已成交
    PartFilled,    // 部分成交
    Inactive,      // 無效
    Failed,        // 失敗
    PendingSubmit, // 等待送出
    PreSubmitted,  // 預先送出
    Submitted,     // 已送出
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderEventType {
    StockDeal,    // 股票成交
    StockOrder,   // 股票委託
    FuturesOrder, // 期貨委託
    FuturesDeal,  // 期貨成交
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuoteType {
    Tick,   // tick
    BidAsk, // 買賣報價
    Quote,  // 報價
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    TWD,
    USD,
    HKD,
    GBP,
    AUD,
    CAD,
    SGD,
    CHF,
    JPY,
    ZAR,
    SEK,
    NZD,
    THB,
    PHP,
    IDR,
    EUR,
    KRW,
    VND,
    MYR,
    CNY,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DayTrade {
    Yes,     // 可當沖
    OnlyBuy, // 只能買
    No,      // 不可當沖
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TickType {
    No,   // 無法判斷
    Buy,  // 外盤
    Sell, // 內盤
}

impl From<i32> for TickType {
    fn from(value: i32) -> Self {
        match value {
            1 => TickType::Buy,
            2 => TickType::Sell,
            _ => TickType::No,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChangeType {
    LimitUp,   // 漲停
    Up,        // 漲
    Unchanged, // 平盤
    Down,      // 跌
    LimitDown, // 跌停
}

impl From<i32> for ChangeType {
    fn from(value: i32) -> Self {
        match value {
            4 => ChangeType::LimitUp,
            2 => ChangeType::Up,
            3 => ChangeType::Unchanged,
            1 => ChangeType::Down,
            5 => ChangeType::LimitDown,
            _ => ChangeType::Unchanged,
        }
    }
}

/// 合約獲取狀態 (對應原始 Python 的 FetchStatus)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FetchStatus {
    Unfetch,  // 未獲取
    Fetching, // 獲取中
    Fetched,  // 已獲取
}
