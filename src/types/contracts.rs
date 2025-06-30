use crate::types::constants::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

/// 合約集合結構 (對應原始 Python 的 Contracts 類別)
///
/// 對應原始 Python 實作：
/// ```python
/// class Contracts:
///     def __init__(self):
///         self.status = FetchStatus.Unfetch
///         self.stocks = {}
///         self.futures = {}
///         self.options = {}
///         self.indices = {}
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contracts {
    /// 獲取狀態
    pub status: FetchStatus,
    /// 股票合約：{code: Contract}
    pub stocks: HashMap<String, Contract>,
    /// 期貨合約：{code: Contract}
    pub futures: HashMap<String, Contract>,
    /// 選擇權合約：{code: Contract}
    pub options: HashMap<String, Contract>,
    /// 指數合約：{code: Contract}
    pub indices: HashMap<String, Contract>,
    /// 更新時間戳
    pub last_updated: chrono::DateTime<chrono::Utc>,
    /// 總合約數量統計
    pub counts: ContractCounts,
}

/// 合約數量統計
///
/// ## 🔍 重要說明：Shioaji Contracts 結構
///
/// 所有合約類型都使用**群組結構**，需要解析群組內的個別合約：
///
/// ### 股票合約結構
/// ```python
/// api.Contracts.Stocks = [
///     TSE(...),  # 台灣證券交易所群組，包含所有上市股票
///     OTC(...),  # 櫃買中心群組，包含所有上櫃股票  
///     OES(...),  # 興櫃群組，包含所有興櫃股票
/// ]
/// ```
///
/// ### 期貨合約結構
/// ```python
/// api.Contracts.Futures = [
///     BRF(...),  # 大台期貨群組，包含各月份合約
///     BTF(...),  # 台指期貨群組，包含各月份合約
///     CAF(...),  # 加權期貨群組，包含各月份合約
///     # ... 359 個商品群組
/// ]
/// ```
///
/// ### 選擇權合約結構  
/// ```python
/// api.Contracts.Options = [
///     CAO(...),  # 加權選擇權群組，包含各履約價和到期月份
///     CBO(...),  # 台指選擇權群組，包含各履約價和到期月份
///     # ... 60 個標的群組
/// ]
/// ```
///
/// ### 指數合約結構
/// ```python
/// api.Contracts.Indexs = [
///     OTC(...),    # 櫃買中心指數群組
///     TAIFEX(...), # 期交所指數群組
///     TSE(...),    # 證交所指數群組
/// ]
/// ```
///
/// 因此正確的統計方法是解析每個群組內的個別合約代碼數量。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContractCounts {
    /// 股票總數 (TSE + OTC + OES)
    pub stocks: i32,
    /// TSE (台灣證券交易所) 上市股票數量
    pub stocks_tse: i32,
    /// OTC (櫃買中心) 上櫃股票數量
    pub stocks_otc: i32,
    /// OES (興櫃) 股票數量
    pub stocks_oes: i32,

    /// 期貨合約總數 (所有商品群組的合約總和)
    pub futures: i32,
    /// 期貨商品群組數量 (BRF, BTF, CAF, etc.)
    pub futures_groups: i32,

    /// 選擇權合約總數 (所有標的群組的合約總和)
    pub options: i32,
    /// 選擇權標的群組數量 (CAO, CBO, CCA, etc.)
    pub options_groups: i32,

    /// 指數總數 (所有交易所的指數總和)
    pub indices: i32,
    /// OTC 指數數量
    pub indices_otc: i32,
    /// TAIFEX 指數數量
    pub indices_taifex: i32,
    /// TSE 指數數量
    pub indices_tse: i32,
}

impl ContractCounts {
    /// 計算總合約數
    pub fn total_count(&self) -> i32 {
        self.stocks + self.futures + self.options + self.indices
    }
}

impl Default for Contracts {
    fn default() -> Self {
        Self::new()
    }
}

impl Contracts {
    /// 創建新的 Contracts 實例 (對應原始 Python 的 new_contracts())
    pub fn new() -> Self {
        Self {
            status: FetchStatus::Unfetch,
            stocks: HashMap::new(),
            futures: HashMap::new(),
            options: HashMap::new(),
            indices: HashMap::new(),
            last_updated: chrono::Utc::now(),
            counts: ContractCounts::default(),
        }
    }

    /// 更新合約數量統計
    pub fn update_counts(&mut self) {
        self.counts.stocks = self.stocks.len() as i32;
        self.counts.futures = self.futures.len() as i32;
        self.counts.options = self.options.len() as i32;
        self.counts.indices = self.indices.len() as i32;
        self.last_updated = chrono::Utc::now();
    }

    /// 加入股票合約
    pub fn add_stock(&mut self, code: String, contract: Contract) {
        self.stocks.insert(code, contract);
    }

    /// 加入期貨合約
    pub fn add_future(&mut self, code: String, contract: Contract) {
        self.futures.insert(code, contract);
    }

    /// 加入選擇權合約
    pub fn add_option(&mut self, code: String, contract: Contract) {
        self.options.insert(code, contract);
    }

    /// 加入指數合約
    pub fn add_index(&mut self, code: String, contract: Contract) {
        self.indices.insert(code, contract);
    }

    /// 取得總合約數
    pub fn total_count(&self) -> i32 {
        self.counts.stocks + self.counts.futures + self.counts.options + self.counts.indices
    }

    /// 檢查是否為空
    pub fn is_empty(&self) -> bool {
        self.total_count() == 0
    }

    /// 重置所有合約
    pub fn clear(&mut self) {
        self.stocks.clear();
        self.futures.clear();
        self.options.clear();
        self.indices.clear();
        self.status = FetchStatus::Unfetch;
        self.counts = ContractCounts::default();
    }
}
