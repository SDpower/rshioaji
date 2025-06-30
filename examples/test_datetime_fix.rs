//! 測試修正後的 datetime 欄位處理
//! 
//! 此程式演示：
//! 1. datetime 欄位不再使用 Utc::now()
//! 2. 使用固定的基準時間 (2024-01-01 09:00:00 UTC)
//! 3. 符合原始 Python shioaji 套件的設計理念

use rshioaji::*;
use chrono::{DateTime, Utc};

fn main() {
    println!("🔧 測試修正後的市場資料結構 datetime 欄位處理");
    println!("========================================");
    
    // 測試所有主要的市場資料結構
    println!("\n📊 創建預設市場資料結構...");
    
    // 1. TickSTKv1 - 股票 Tick 資料
    let tick_stk = TickSTKv1::default();
    println!("✅ TickSTKv1 datetime: {}", tick_stk.datetime);
    println!("   - 不再使用當前時間，使用固定基準時間");
    
    // 2. TickFOPv1 - 期貨/選擇權 Tick 資料
    let tick_fop = TickFOPv1::default();
    println!("✅ TickFOPv1 datetime: {}", tick_fop.datetime);
    
    // 3. BidAskSTKv1 - 股票買賣五檔資料
    let bidask_stk = BidAskSTKv1::default();
    println!("✅ BidAskSTKv1 datetime: {}", bidask_stk.datetime);
    println!("   - 新增 intraday_odd 欄位: {}", bidask_stk.intraday_odd);
    
    // 4. BidAskFOPv1 - 期貨/選擇權買賣五檔資料
    let bidask_fop = BidAskFOPv1::default();
    println!("✅ BidAskFOPv1 datetime: {}", bidask_fop.datetime);
    println!("   - 新增 bid_total_vol: {}", bidask_fop.bid_total_vol);
    println!("   - 新增 ask_total_vol: {}", bidask_fop.ask_total_vol);
    println!("   - 新增 first_derived_bid_price: {}", bidask_fop.first_derived_bid_price);
    
    // 5. QuoteSTKv1 - 股票報價資料 (已完全重構符合原始定義)
    let quote_stk = QuoteSTKv1::default();
    println!("✅ QuoteSTKv1 datetime: {}", quote_stk.datetime);
    println!("   - 完全重構符合原始 Python 定義");
    println!("   - 新增 closing_oddlot_* 系列欄位");
    println!("   - 新增 fixed_trade_amount: {}", quote_stk.fixed_trade_amount);
    
    println!("\n🎯 關鍵改進摘要:");
    println!("================");
    println!("✅ datetime 不再使用 Utc::now()，改用固定基準時間");
    println!("✅ TickSTKv1 新增: closing_oddlot_shares, fixed_trade_vol, intraday_odd");
    println!("✅ BidAskSTKv1 新增: intraday_odd");
    println!("✅ BidAskFOPv1 新增: bid_total_vol, ask_total_vol, first_derived_* 系列欄位");
    println!("✅ QuoteSTKv1 完全重構，符合原始 Python 定義的 39 個欄位");
    
    // 驗證所有結構的 datetime 都相同（使用相同基準時間）
    let expected_time: DateTime<Utc> = "2024-01-01T09:00:00Z".parse().unwrap();
    
    assert_eq!(tick_stk.datetime, expected_time);
    assert_eq!(tick_fop.datetime, expected_time);
    assert_eq!(bidask_stk.datetime, expected_time);
    assert_eq!(bidask_fop.datetime, expected_time);
    assert_eq!(quote_stk.datetime, expected_time);
    
    println!("\n✅ 所有驗證通過！datetime 欄位已正確修正");
    println!("🎉 現在符合原始 Python shioaji 套件的設計理念");
}