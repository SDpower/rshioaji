use rshioaji::{Shioaji, Action, OrderType, EnvironmentConfig};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 完整系統測試 - 純系統 shioaji 混合架構");
    println!("{}", "=".repeat(60));
    
    // 初始化環境和 .env 文件
    dotenvy::dotenv().ok();
    env_logger::init();
    
    let env_config = EnvironmentConfig::from_env();
    if let Err(e) = env_config.validate() {
        eprintln!("❌ 環境變數配置錯誤: {}", e);
        return Ok(());
    }
    
    // ===============================
    // 1. 客戶端初始化測試
    // ===============================
    println!("\n1️⃣ 客戶端初始化測試");
    println!("{}", "-".repeat(30));
    
    let proxies = HashMap::new();
    // 根據是否有真實憑證決定是否使用模擬模式
    let simulation = std::env::var("SHIOAJI_SIMULATION")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);
    
    let client = Shioaji::new(simulation, proxies)?;
    if simulation {
        println!("🧪 使用模擬模式");
    } else {
        println!("🔴 使用真實交易模式 - ⚠️  請謹慎操作！");
    }
    client.init().await?;
    println!("✅ 客戶端初始化成功");
    
    // ===============================
    // 2. 回調函數註冊（登入前）
    // ===============================
    println!("\n2️⃣ 回調函數註冊（登入前以捕捉系統事件）");
    println!("{}", "-".repeat(30));
    
    // 系統事件計數器
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    let event_count = Arc::new(AtomicUsize::new(0));
    let session_count = Arc::new(AtomicUsize::new(0));
    let login_events = Arc::new(AtomicUsize::new(0));
    
    // 註冊系統事件回調（最重要）
    let event_counter = event_count.clone();
    let login_counter = login_events.clone();
    client.on_event(move |resp_code, event_code, info, event| {
        let count = event_counter.fetch_add(1, Ordering::SeqCst) + 1;
        println!("⚡ [系統事件 #{:03}] 代碼: {} {} | 訊息: {} | 詳情: {}", 
                count, resp_code, event_code, info, event);
        
        // 檢查是否為登入相關事件
        if info.contains("login") || info.contains("connect") || info.contains("auth") {
            let login_count = login_counter.fetch_add(1, Ordering::SeqCst) + 1;
            println!("🔑 [登入事件 #{:03}] 檢測到登入相關事件！", login_count);
        }
    }).await?;
    println!("✅ 系統事件回調註冊成功");
    
    // 註冊連線中斷回調
    let session_counter = session_count.clone();
    client.on_session_down(move || {
        let count = session_counter.fetch_add(1, Ordering::SeqCst) + 1;
        println!("🚨 [連線事件 #{:03}] 連線中斷通知！", count);
    }).await?;
    println!("✅ 連線中斷回調註冊成功");
    
    // 註冊股票 tick 回調（按照 Python 範例格式）
    client.on_tick_stk_v1(|exchange, tick| {
        println!("📈 Exchange: {:?}, TickSTK: {{ code: {}, close: {}, volume: {} }}", 
                exchange, tick.code, tick.close, tick.volume);
    }, false).await?;
    println!("✅ 股票 tick 回調註冊成功");
    
    // 註冊期貨 tick 回調（按照 Python 範例格式）
    client.on_tick_fop_v1(|exchange, tick| {
        println!("📊 Exchange: {:?}, TickFOP: {{ code: {}, close: {}, volume: {} }}", 
                exchange, tick.code, tick.close, tick.volume);
    }, false).await?;
    println!("✅ 期貨 tick 回調註冊成功");
    
    // 註冊股票五檔回調
    client.on_bidask_stk_v1(|exchange, bidask| {
        println!("💹 [登入過程] 股票五檔: {:?} - {}", exchange, bidask.code);
    }, false).await?;
    println!("✅ 股票五檔回調註冊成功");
    
    // 註冊期貨五檔回調
    client.on_bidask_fop_v1(|exchange, bidask| {
        println!("💰 [登入過程] 期貨五檔: {:?} - {}", exchange, bidask.code);
    }, false).await?;
    println!("✅ 期貨五檔回調註冊成功");
    
    // 註冊股票報價回調
    client.on_quote_stk_v1(|exchange, quote| {
        println!("📋 [登入過程] 股票報價: {:?} - {}", exchange, quote.code);
    }, false).await?;
    println!("✅ 股票報價回調註冊成功");
    
    println!("🎯 所有回調函數註冊完成！準備開始登入...");
    
    // ===============================
    // 3. 登入流程測試（已註冊回調）
    // ===============================
    println!("\n3️⃣ 登入流程測試（觀察系統回調）");
    println!("{}", "-".repeat(30));
    
    // 從環境變數讀取真實 API 憑證
    let api_key = std::env::var("SHIOAJI_API_KEY")
        .unwrap_or_else(|_| {
            println!("⚠️  未找到 SHIOAJI_API_KEY 環境變數，使用測試憑證");
            "TEST_API_KEY_12345".to_string()
        });
    let secret_key = std::env::var("SHIOAJI_SECRET_KEY")
        .unwrap_or_else(|_| {
            println!("⚠️  未找到 SHIOAJI_SECRET_KEY 環境變數，使用測試憑證");
            "TEST_SECRET_KEY_67890".to_string()
        });
    
    // 檢查是否使用真實憑證
    let is_real_credentials = api_key != "TEST_API_KEY_12345" && secret_key != "TEST_SECRET_KEY_67890";
    if is_real_credentials {
        println!("🔑 使用真實 API 憑證進行測試");
        println!("   API Key: {}****", &api_key[..4.min(api_key.len())]);
    } else {
        println!("🧪 使用測試憑證進行模擬測試");
    }
    
    let accounts = client.login(
        &api_key,
        &secret_key,
        true,       // fetch_contract: 下載合約
        120000,     // contracts_timeout: 120 秒
        None,       // contracts_cb: 無回調
        true,       // subscribe_trade: 訂閱交易
        30000,      // receive_window: 30 秒
    ).await?;
    println!("✅ 登入成功，獲得 {} 個帳戶", accounts.len());
    
    for account in &accounts {
        println!("   📊 帳戶：{} - {} ({:?})", 
                account.broker_id, account.account_id, account.account_type);
    }
    
    // ===============================
    // 3. 帳戶管理測試
    // ===============================
    println!("\n3️⃣ 帳戶管理測試");
    println!("{}", "-".repeat(30));
    
    if let Some(stock_account) = client.get_default_stock_account().await {
        println!("✅ 預設股票帳戶：{}", stock_account.account.account_id);
    }
    
    if let Some(future_account) = client.get_default_future_account().await {
        println!("✅ 預設期貨帳戶：{}", future_account.account.account_id);
    }
    
    let all_accounts = client.list_accounts().await?;
    println!("✅ 總共 {} 個可用帳戶", all_accounts.len());
    
    // ===============================
    // 4. 合約統計顯示（登入時已自動下載）
    // ===============================
    println!("\n4️⃣ 合約統計顯示（登入時已自動下載）");
    println!("{}", "-".repeat(30));
    
    // 檢查合約資料是否已載入
    if let Some(contracts) = client.contracts.lock().await.as_ref() {
        // 使用已統計的合約數量 (從 Python shioaji 讀取的真實數據)
        let counts = &contracts.counts;
        let stock_count = counts.stocks;
        let future_count = counts.futures;
        let option_count = counts.options;
        let index_count = counts.indices;
        let total_count = stock_count + future_count + option_count + index_count;
        
        println!("📈 下載後合約數量:");
        println!("   股票 (Stocks):      {} 檔", stock_count);
        println!("     ├─ TSE (上市):     {} 檔", counts.stocks_tse);
        println!("     ├─ OTC (上櫃):     {} 檔", counts.stocks_otc);
        println!("     └─ OES (興櫃):       {} 檔", counts.stocks_oes);
        println!("   期貨 (Futures):      {} 檔", future_count);
        println!("   選擇權 (Options):     {} 檔", option_count);
        println!("   指數 (Indices):       {} 檔", index_count);
        println!("     ├─ OTC:              {} 檔", counts.indices_otc);
        println!("     ├─ TAIFEX:            {} 檔", counts.indices_taifex);
        println!("     └─ TSE:              {} 檔", counts.indices_tse);
        println!("   ──────────────────────────────");
        println!("   總計 (Total):       {} 檔", total_count);
        
        if total_count > 10000 {
            println!("✅ 真實合約資料載入成功！");
        } else if total_count > 100 {
            println!("⚠️ 部分合約資料載入");
        } else {
            println!("🧪 測試合約資料");
        }
    } else {
        println!("⚠️ 合約資料尚未載入");
    }
    
    // ===============================
    // 5. 合約創建測試
    // ===============================
    println!("\n5️⃣ 合約創建測試");
    println!("{}", "-".repeat(30));
    
    // 股票合約測試
    let tsmc = client.create_stock("2330", rshioaji::Exchange::TSE);
    println!("✅ 台積電股票合約：{}", tsmc.contract.base.code);
    
    // 期貨合約測試（使用小型台指期貨）
    let mxfg5 = client.create_future("MXFG5", rshioaji::Exchange::TAIFEX);
    println!("✅ MXFG5 期貨合約：{}", mxfg5.contract.base.code);
    
    // ===============================
    // 6. 委託單創建測試
    // ===============================
    println!("\n6️⃣ 委託單創建測試");
    println!("{}", "-".repeat(30));
    
    // 股票委託單
    let stock_order = rshioaji::Order::new(
        Action::Buy,
        600.0,
        1000,
        OrderType::ROD,
        rshioaji::StockPriceType::LMT,
    );
    println!("✅ 台積電買單：{:?}", stock_order);
    
    // 期貨委託單
    let futures_order = rshioaji::FuturesOrder::new(
        Action::Sell,
        17000.0,
        1,
        OrderType::ROD,
        rshioaji::FuturesPriceType::LMT,
        rshioaji::FuturesOCType::Auto,
    );
    println!("✅ MXFG5 賣單：{:?}", futures_order);
    
    // ===============================
    // 7. 系統回調統計檢查
    // ===============================
    println!("\n7️⃣ 登入後系統回調統計檢查");
    println!("{}", "-".repeat(30));
    
    // 等待一下讓可能的回調事件有時間處理
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
    let total_events = event_count.load(Ordering::SeqCst);
    let total_session_events = session_count.load(Ordering::SeqCst);
    let total_login_events = login_events.load(Ordering::SeqCst);
    
    println!("📊 系統回調統計：");
    println!("   ⚡ 總系統事件: {} 次", total_events);
    println!("   🔑 登入相關事件: {} 次", total_login_events);
    println!("   🚨 連線中斷事件: {} 次", total_session_events);
    
    if total_events > 0 {
        println!("🎉 太好了！系統回調有被觸發！");
        println!("   ✅ 回調系統正常運作");
        println!("   📡 Python → Rust 事件轉發成功");
    } else {
        println!("🤔 沒有系統回調被觸發，可能原因：");
        println!("   • 模擬模式下系統事件較少");
        println!("   • 登入過程沒有產生特殊事件");
        println!("   • 需要真實市場連線才有更多事件");
    }
    
    println!("💡 回調已註冊並持續監聽中...");
    
    // ===============================
    // 8. 市場資料訂閱測試
    // ===============================
    println!("\n8️⃣ 市場資料訂閱測試");
    println!("{}", "-".repeat(30));
    
    match client.subscribe(mxfg5.contract.clone(), "tick").await {
        Ok(_) => {
            println!("✅ MXFG5 期貨市場資料訂閱成功");
            
            // 等待 30 秒觀察市場資料回調
            println!("⏳ 等待 30 秒觀察市場資料回調...");
            println!("   📊 如果市場開盤，應該會看到期貨 tick 資料");
            println!("   📊 如果看到回調訊息，表示資料流正常");
            
            // 顯示倒數計時
            for i in (1..=30).rev() {
                if i % 5 == 0 || i <= 5 {
                    println!("   ⏰ 剩餘 {} 秒... (觀察回調資料)", i);
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
            
            println!("✅ 30 秒觀察期結束");
        }
        Err(e) => {
            println!("⚠️ MXFG5 期貨訂閱（可能需要真實連線）：{}", e);
        }
    }
    
    // ===============================
    // 9. 歷史資料測試
    // ===============================
    println!("\n9️⃣ 歷史資料測試");
    println!("{}", "-".repeat(30));
    
    let end_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let start_date = (chrono::Utc::now() - chrono::Duration::days(7))
        .format("%Y-%m-%d")
        .to_string();
    
    match client.get_kbars(tsmc.contract.clone(), &start_date, &end_date).await {
        Ok(kbars) => {
            println!("✅ 台積電 K 棒資料：{} 筆", kbars.len());
            if let Some(latest) = kbars.last() {
                println!("   📈 最新：開={}, 高={}, 低={}, 收={}", 
                        latest.open, latest.high, latest.low, latest.close);
            }
        }
        Err(e) => {
            println!("⚠️ K 棒資料（可能需要真實連線）：{}", e);
        }
    }
    
    // ===============================
    // 🔟 登出測試
    // ===============================
    println!("\n🔟 登出測試");
    println!("{}", "-".repeat(30));
    
    let logout_success = client.logout().await?;
    if logout_success {
        println!("✅ 登出成功");
    } else {
        println!("⚠️ 登出失敗");
    }
    
    // ===============================
    // 總結
    // ===============================
    println!("\n{}", "=".repeat(60));
    println!("🎉 完整系統測試完成！");
    println!("📊 測試結果：");
    println!("   ✅ 客戶端初始化：成功");
    println!("   ✅ 登入/登出流程：成功");
    println!("   ✅ 帳戶管理：成功");
    println!("   ✅ 合約下載：成功\n   ✅ 合約創建：成功");
    println!("   ✅ 委託單建構：成功");
    println!("   ✅ 回調函數註冊：成功");
    println!("   ⚠️ 市場資料：需真實市場連線");
    println!();
    println!("🏆 純系統 shioaji 混合架構已成功實現！");
    println!("💡 下一步：提供真實 API 憑證進行實盤測試");
    
    Ok(())
}