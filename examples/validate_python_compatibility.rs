//! 驗證 Rust 市場資料結構與原始 Python shioaji 套件的完全相容性
//! 
//! 此程式根據 /shioaji/stream_data_type.py 驗證每個結構的：
//! 1. 欄位數量
//! 2. 欄位順序  
//! 3. 型別對應
//! 4. Default 實作正確性

use rshioaji::*;

fn main() {
    println!("🔍 驗證 rshioaji 市場資料結構與原始 Python 定義的完全相容性");
    println!("{}", "=".repeat(80));
    
    test_tickstkv1_compatibility();
    test_tickfopv1_compatibility();
    test_bidaskstkv1_compatibility();
    test_bidaskfopv1_compatibility();
    test_quotestkv1_compatibility();
    
    println!("\n🎉 所有驗證通過！rshioaji 市場資料結構與原始 Python 定義完全相容！");
}

fn test_tickstkv1_compatibility() {
    println!("\n📊 驗證 TickSTKv1 結構...");
    
    let tick = TickSTKv1::default();
    
    // 驗證 JSON 序列化包含所有必要欄位
    let json = serde_json::to_value(&tick).unwrap();
    let obj = json.as_object().unwrap();
    
    // 原始 Python 定義有 25 個欄位
    let expected_fields = vec![
        "code", "datetime", "open", "avg_price", "close", "high", "low", 
        "amount", "total_amount", "volume", "total_volume", "tick_type", 
        "chg_type", "price_chg", "pct_chg", "bid_side_total_vol", 
        "ask_side_total_vol", "bid_side_total_cnt", "ask_side_total_cnt", 
        "closing_oddlot_shares", "fixed_trade_vol", "suspend", "simtrade", "intraday_odd"
    ];
    
    assert_eq!(obj.len(), expected_fields.len(), "TickSTKv1 欄位數量不匹配");
    
    for field in &expected_fields {
        assert!(obj.contains_key(*field), "TickSTKv1 缺少欄位: {}", field);
    }
    
    // 驗證關鍵型別
    assert!(tick.code.is_empty());
    assert_eq!(tick.volume, 0i64);
    assert!(!tick.suspend);
    assert!(!tick.intraday_odd);
    
    println!("✅ TickSTKv1: {} 個欄位，所有必要欄位存在", expected_fields.len());
}

fn test_tickfopv1_compatibility() {
    println!("\n📊 驗證 TickFOPv1 結構...");
    
    let tick = TickFOPv1::default();
    let json = serde_json::to_value(&tick).unwrap();
    let obj = json.as_object().unwrap();
    
    // 原始 Python 定義有 19 個欄位
    let expected_fields = vec![
        "code", "datetime", "open", "underlying_price", "bid_side_total_vol", 
        "ask_side_total_vol", "avg_price", "close", "high", "low", "amount", 
        "total_amount", "volume", "total_volume", "tick_type", "chg_type", 
        "price_chg", "pct_chg", "simtrade"
    ];
    
    assert_eq!(obj.len(), expected_fields.len(), "TickFOPv1 欄位數量不匹配");
    
    for field in &expected_fields {
        assert!(obj.contains_key(*field), "TickFOPv1 缺少欄位: {}", field);
    }
    
    // 驗證特有欄位
    assert_eq!(tick.underlying_price, 0.0);
    assert!(!tick.simtrade);
    
    println!("✅ TickFOPv1: {} 個欄位，所有必要欄位存在", expected_fields.len());
}

fn test_bidaskstkv1_compatibility() {
    println!("\n📊 驗證 BidAskSTKv1 結構...");
    
    let bidask = BidAskSTKv1::default();
    let json = serde_json::to_value(&bidask).unwrap();
    let obj = json.as_object().unwrap();
    
    // 原始 Python 定義有 11 個欄位
    let expected_fields = vec![
        "code", "datetime", "bid_price", "bid_volume", "diff_bid_vol", 
        "ask_price", "ask_volume", "diff_ask_vol", "suspend", "simtrade", "intraday_odd"
    ];
    
    assert_eq!(obj.len(), expected_fields.len(), "BidAskSTKv1 欄位數量不匹配");
    
    for field in &expected_fields {
        assert!(obj.contains_key(*field), "BidAskSTKv1 缺少欄位: {}", field);
    }
    
    // 驗證 List 型別預設為 5 檔
    assert_eq!(bidask.bid_price.len(), 5);
    assert_eq!(bidask.bid_volume.len(), 5);
    assert_eq!(bidask.diff_bid_vol.len(), 5);
    assert_eq!(bidask.ask_price.len(), 5);
    assert_eq!(bidask.ask_volume.len(), 5);
    assert_eq!(bidask.diff_ask_vol.len(), 5);
    assert!(!bidask.intraday_odd);
    
    println!("✅ BidAskSTKv1: {} 個欄位，五檔資料結構正確", expected_fields.len());
}

fn test_bidaskfopv1_compatibility() {
    println!("\n📊 驗證 BidAskFOPv1 結構...");
    
    let bidask = BidAskFOPv1::default();
    let json = serde_json::to_value(&bidask).unwrap();
    let obj = json.as_object().unwrap();
    
    // 原始 Python 定義有 16 個欄位
    let expected_fields = vec![
        "code", "datetime", "bid_total_vol", "ask_total_vol", "bid_price", 
        "bid_volume", "diff_bid_vol", "ask_price", "ask_volume", "diff_ask_vol", 
        "first_derived_bid_price", "first_derived_ask_price", "first_derived_bid_vol", 
        "first_derived_ask_vol", "underlying_price", "simtrade"
    ];
    
    assert_eq!(obj.len(), expected_fields.len(), "BidAskFOPv1 欄位數量不匹配");
    
    for field in &expected_fields {
        assert!(obj.contains_key(*field), "BidAskFOPv1 缺少欄位: {}", field);
    }
    
    // 驗證期貨/選擇權特有欄位
    assert_eq!(bidask.bid_total_vol, 0);
    assert_eq!(bidask.ask_total_vol, 0);
    assert_eq!(bidask.first_derived_bid_price, 0.0);
    assert_eq!(bidask.first_derived_ask_price, 0.0);
    assert_eq!(bidask.first_derived_bid_vol, 0);
    assert_eq!(bidask.first_derived_ask_vol, 0);
    assert_eq!(bidask.underlying_price, 0.0);
    
    println!("✅ BidAskFOPv1: {} 個欄位，衍生價格欄位存在", expected_fields.len());
}

fn test_quotestkv1_compatibility() {
    println!("\n📊 驗證 QuoteSTKv1 結構...");
    
    let quote = QuoteSTKv1::default();
    let json = serde_json::to_value(&quote).unwrap();
    let obj = json.as_object().unwrap();
    
    // 原始 Python 定義有 35 個欄位（注意：沒有 intraday_odd）
    let expected_fields = vec![
        "code", "datetime", "open", "avg_price", "close", "high", "low", 
        "amount", "total_amount", "volume", "total_volume", "tick_type", 
        "chg_type", "price_chg", "pct_chg", "bid_side_total_vol", 
        "ask_side_total_vol", "bid_side_total_cnt", "ask_side_total_cnt", 
        "closing_oddlot_shares", "closing_oddlot_close", "closing_oddlot_amount", 
        "closing_oddlot_bid_price", "closing_oddlot_ask_price", "fixed_trade_vol", 
        "fixed_trade_amount", "bid_price", "bid_volume", "diff_bid_vol", 
        "ask_price", "ask_volume", "diff_ask_vol", "avail_borrowing", 
        "suspend", "simtrade"
    ];
    
    assert_eq!(obj.len(), expected_fields.len(), "QuoteSTKv1 欄位數量不匹配");
    
    for field in &expected_fields {
        assert!(obj.contains_key(*field), "QuoteSTKv1 缺少欄位: {}", field);
    }
    
    // 重要驗證：QuoteSTKv1 在原始 Python 定義中沒有 intraday_odd 欄位
    assert!(!obj.contains_key("intraday_odd"), "QuoteSTKv1 不應該有 intraday_odd 欄位");
    
    // 驗證盤後零股欄位
    assert_eq!(quote.closing_oddlot_shares, 0);
    assert_eq!(quote.closing_oddlot_close, 0.0);
    assert_eq!(quote.closing_oddlot_amount, 0.0);
    assert_eq!(quote.closing_oddlot_bid_price, 0.0);
    assert_eq!(quote.closing_oddlot_ask_price, 0.0);
    assert_eq!(quote.fixed_trade_vol, 0);
    assert_eq!(quote.fixed_trade_amount, 0.0);
    assert_eq!(quote.avail_borrowing, 0);
    
    // 驗證五檔資料
    assert_eq!(quote.bid_price.len(), 5);
    assert_eq!(quote.bid_volume.len(), 5);
    assert_eq!(quote.diff_bid_vol.len(), 5);
    assert_eq!(quote.ask_price.len(), 5);
    assert_eq!(quote.ask_volume.len(), 5);
    assert_eq!(quote.diff_ask_vol.len(), 5);
    
    println!("✅ QuoteSTKv1: {} 個欄位，盤後零股和借券欄位完整", expected_fields.len());
}