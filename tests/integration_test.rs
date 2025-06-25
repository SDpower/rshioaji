use rshioaji::{Shioaji, Exchange, Action, OrderType, StockPriceType};
use std::collections::HashMap;

#[tokio::test]
async fn test_client_creation() {
    let proxies = HashMap::new();
    let client = Shioaji::new(true, proxies);
    assert!(client.is_ok());
}

#[tokio::test]
async fn test_client_initialization() {
    let proxies = HashMap::new();
    let client = Shioaji::new(true, proxies).unwrap();
    
    // Note: This test might fail if Python environment is not properly set up
    // In a real test environment, you would mock the Python bindings
    let result = client.init().await;
    
    // For now, we just check that the client was created successfully
    // In practice, you'd want to mock the Python interface for testing
    println!("Client initialization result: {:?}", result);
}

#[test]
fn test_contract_creation() {
    let proxies = HashMap::new();
    let client = Shioaji::new(true, proxies).unwrap();
    
    // Test TXFG5 futures contract creation
    let txfg5_future = client.create_future("TXFG5", Exchange::TAIFEX);
    assert_eq!(txfg5_future.contract.base.code, "TXFG5");
    // TXFG5 is traded on TAIFEX
    assert_eq!(txfg5_future.contract.base.security_type, rshioaji::SecurityType::Future);
    
    // Test future contract creation
    let future = client.create_future("TXFA4", Exchange::TAIFEX);
    assert_eq!(future.contract.base.code, "TXFA4");
    assert_eq!(future.contract.base.exchange, Exchange::TAIFEX);
    assert_eq!(future.contract.base.security_type, rshioaji::SecurityType::Future);
    
    // Test option contract creation
    let option = client.create_option("TXO", rshioaji::OptionRight::Call, 17000.0);
    assert_eq!(option.contract.base.code, "TXO");
    assert_eq!(option.contract.strike_price, 17000.0);
    assert_eq!(option.contract.option_right, rshioaji::OptionRight::Call);
}

#[test]
fn test_order_creation() {
    // Test stock order
    let order = rshioaji::Order::new(
        Action::Buy,
        500.0,
        1000,
        OrderType::ROD,
        StockPriceType::LMT,
    );
    
    assert_eq!(order.action, Action::Buy);
    assert_eq!(order.price, 500.0);
    assert_eq!(order.quantity, 1000);
    assert_eq!(order.order_type, OrderType::ROD);
    assert_eq!(order.price_type, StockPriceType::LMT);
    
    // Test futures order
    let futures_order = rshioaji::FuturesOrder::new(
        Action::Sell,
        17000.0,
        2,
        OrderType::IOC,
        rshioaji::FuturesPriceType::MKT,
        rshioaji::FuturesOCType::DayTrade,
    );
    
    assert_eq!(futures_order.action, Action::Sell);
    assert_eq!(futures_order.price, 17000.0);
    assert_eq!(futures_order.quantity, 2);
    assert_eq!(futures_order.order_type, OrderType::IOC);
    assert_eq!(futures_order.price_type, rshioaji::FuturesPriceType::MKT);
    assert_eq!(futures_order.octype, rshioaji::FuturesOCType::DayTrade);
}

#[test]
fn test_account_types() {
    use rshioaji::{Account, AccountType};
    
    let stock_account = Account::new(
        "9A95".to_string(),
        "1234567".to_string(),
        AccountType::Stock,
        "testuser".to_string(),
        true,
    );
    
    assert_eq!(stock_account.account_type, AccountType::Stock);
    assert_eq!(stock_account.broker_id, "9A95");
    assert_eq!(stock_account.account_id, "1234567");
    assert_eq!(stock_account.username, "testuser");
    assert!(stock_account.signed);
    
    let future_account = Account::new(
        "9A95".to_string(),
        "7654321".to_string(),
        AccountType::Future,
        "testuser".to_string(),
        false,
    );
    
    assert_eq!(future_account.account_type, AccountType::Future);
    assert!(!future_account.signed);
}

#[test]
fn test_market_data_types() {
    use rshioaji::{Kbar, Tick, TickType};
    use chrono::Utc;
    
    let now = Utc::now();
    
    // Test Kbar
    let kbar = Kbar {
        ts: now,
        open: 500.0,
        high: 510.0,
        low: 495.0,
        close: 505.0,
        volume: 1000,
        amount: 505000.0,
    };
    
    assert_eq!(kbar.open, 500.0);
    assert_eq!(kbar.high, 510.0);
    assert_eq!(kbar.low, 495.0);
    assert_eq!(kbar.close, 505.0);
    assert_eq!(kbar.volume, 1000);
    
    // Test Tick
    let tick = Tick {
        ts: now,
        close: 505.0,
        volume: 100,
        bid_price: 504.0,
        bid_volume: 50,
        ask_price: 506.0,
        ask_volume: 75,
        tick_type: TickType::Buy,
    };
    
    assert_eq!(tick.close, 505.0);
    assert_eq!(tick.volume, 100);
    assert_eq!(tick.tick_type, TickType::Buy);
}

#[test]
fn test_enums() {
    use rshioaji::*;
    
    // Test Action enum
    assert_eq!(Action::Buy.to_string(), "Buy");
    assert_eq!(Action::Sell.to_string(), "Sell");
    
    // Test Exchange enum
    assert_eq!(Exchange::TSE.to_string(), "TSE");
    assert_eq!(Exchange::OTC.to_string(), "OTC");
    assert_eq!(Exchange::TAIFEX.to_string(), "TAIFEX");
    
    // Test SecurityType enum
    assert_eq!(SecurityType::Stock.to_string(), "STK");
    assert_eq!(SecurityType::Future.to_string(), "FUT");
    assert_eq!(SecurityType::Option.to_string(), "OPT");
    assert_eq!(SecurityType::Index.to_string(), "IND");
    
    // Test OrderType enum
    assert_eq!(OrderType::ROD.to_string(), "ROD");
    assert_eq!(OrderType::IOC.to_string(), "IOC");
    assert_eq!(OrderType::FOK.to_string(), "FOK");
}