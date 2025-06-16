use rshioaji::{Shioaji, Exchange, Action, OrderType, StockPriceType, EnvironmentConfig, init_logging};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 📚 前置作業：初始化環境配置和日誌系統
    // 對應 Python shioaji utils.py 的功能
    println!("🔧 正在初始化 rshioaji 環境...");
    
    // 載入環境變數配置
    let env_config = EnvironmentConfig::from_env();
    if let Err(e) = env_config.validate() {
        eprintln!("❌ 環境變數配置錯誤: {}", e);
        return Ok(());
    }
    
    println!("📋 環境配置: {}", env_config.summary());
    
    // 初始化日誌系統（對應 Python 的 log 設定）
    if let Err(e) = init_logging(&env_config) {
        eprintln!("❌ 日誌系統初始化失敗: {}", e);
        // 使用基本的 env_logger 作為備用
        env_logger::init();
    }
    
    log::info!("🚀 rshioaji 環境初始化完成");
    log::info!("📊 日誌等級: {}", env_config.log_level);
    log::info!("🛡️  Sentry 錯誤追蹤: {}", if env_config.log_sentry { "啟用" } else { "停用" });
    log::info!("📁 日誌檔案路徑: {}", env_config.sj_log_path);
    log::info!("🧪 遺留測試模式: {}", env_config.legacy_test);
    
    // 顯示平台資訊
    let platform = rshioaji::platform::Platform::detect();
    log::info!("🖥️  偵測到的平台：{:?}", platform);
    println!("🖥️  偵測到的平台：{:?}", platform);
    
    if let Some(platform_dir) = platform.directory_name() {
        log::info!("📁 使用平台目錄：{}", platform_dir);
        println!("📁 使用平台目錄：{}", platform_dir);
        
        // 驗證安裝
        let base_path = std::env::current_dir()?;
        match platform.validate_installation(&base_path) {
            Ok(()) => {
                log::info!("✅ 平台安裝驗證成功");
                println!("✅ 平台安裝驗證成功");
            },
            Err(e) => {
                log::error!("❌ 平台驗證失敗：{}", e);
                println!("❌ 平台驗證失敗：{}", e);
                println!("💡 請確保您的平台有正確的 shioaji 函式庫");
                return Ok(());
            }
        }
    } else {
        log::error!("❌ 不支援的平台");
        println!("❌ 不支援的平台");
        return Ok(());
    }
    
    // 建立 Shioaji 客戶端（模擬模式）
    let proxies = HashMap::new();
    let client = Shioaji::new(true, proxies)?;
    
    // 初始化客戶端
    client.init().await?;
    log::info!("✅ Shioaji 客戶端初始化成功");
    println!("✅ Shioaji 客戶端初始化成功");
    
    // 注意：請替換為您的實際 API 憑證
    let _api_key = "YOUR_API_KEY";
    let _secret_key = "YOUR_SECRET_KEY";
    
    // 💡 完整的登入流程示範
    // 根據 shioaji Python 原始碼，標準登入流程包括：
    // 1. 調用 token_login 或 simulation_login
    // 2. 獲取 accounts 和 contract_download 資訊
    // 3. 設定錯誤追蹤 (error_tracking)
    // 4. 如果 fetch_contract 為 true，則獲取合約資料
    // 5. 設定預設股票和期貨帳戶
    
    // 登入（若沒有憑證請註解掉）
    /*
    log::info!("🔑 開始登入流程...");
    println!("\n🔑 開始登入流程...");
    println!("📋 執行標準登入步驟：");
    println!("   1️⃣  調用 login 方法 (內部會呼叫 token_login 或 simulation_login)");
    println!("   2️⃣  獲取帳戶清單和合約下載資訊");
    println!("   3️⃣  設定錯誤追蹤系統");
    println!("   4️⃣  下載合約資料 (fetch_contract=true)");
    println!("   5️⃣  設定預設股票和期貨帳戶");
    
    // 步驟 1-5：呼叫 login 方法（內部會執行完整的登入流程）
    let accounts = client.login(_api_key, _secret_key, true).await?;
    log::info!("✅ 登入成功！找到 {} 個帳戶", accounts.len());
    println!("✅ 登入成功！找到 {} 個帳戶", accounts.len());
    
    // 顯示帳戶資訊
    for (i, account) in accounts.iter().enumerate() {
        let account_info = format!(
            "📊 帳戶 {} - ID: {} ({}), 類型: {:?}, 已簽署: {}",
            i + 1, account.account_id, account.username, account.account_type, account.signed
        );
        log::info!("{}", account_info);
        println!("{}", account_info);
        
        // 根據帳戶類型顯示詳細資訊
        match account.account_type {
            rshioaji::AccountType::Stock => {
                log::debug!("   🏦 股票帳戶 - 可進行證券交易");
                println!("   🏦 股票帳戶 - 可進行證券交易");
            },
            rshioaji::AccountType::Future => {
                log::debug!("   🔮 期貨帳戶 - 可進行期貨/選擇權交易");
                println!("   🔮 期貨帳戶 - 可進行期貨/選擇權交易");
            }
        }
    }
    
    // 步驟 6：檢查並設定預設帳戶
    log::info!("🔧 檢查預設帳戶設定...");
    println!("\n🔧 檢查預設帳戶設定...");
    if let Some(stock_account) = client.get_default_stock_account().await {
        let msg = format!("✅ 預設股票帳戶：{}", stock_account.account.account_id);
        log::info!("{}", msg);
        println!("{}", msg);
    } else {
        log::warn!("⚠️  尚未設定預設股票帳戶");
        println!("⚠️  尚未設定預設股票帳戶");
    }
    
    if let Some(future_account) = client.get_default_future_account().await {
        let msg = format!("✅ 預設期貨帳戶：{}", future_account.account.account_id);
        log::info!("{}", msg);
        println!("{}", msg);
    } else {
        log::warn!("⚠️  尚未設定預設期貨帳戶");
        println!("⚠️  尚未設定預設期貨帳戶");
    }
    
    // 步驟 7：列出所有可用帳戶
    log::info!("📋 列出所有可用帳戶...");
    println!("\n📋 列出所有可用帳戶...");
    let all_accounts = client.list_accounts().await?;
    let summary = format!("總共有 {} 個可用帳戶", all_accounts.len());
    log::info!("{}", summary);
    println!("{}", summary);
    
    for account in &all_accounts {
        let account_info = format!(
            "   - {} ({}) - {} 帳戶",
            account.account_id,
            account.username,
            match account.account_type {
                rshioaji::AccountType::Stock => "股票",
                rshioaji::AccountType::Future => "期貨",
            }
        );
        log::debug!("{}", account_info);
        println!("{}", account_info);
    }
    */
    
    // 建立範例合約
    log::info!("📈 建立範例合約...");
    println!("\n📈 建立範例合約...");
    
    // 台積電 (2330)
    let tsmc = client.create_stock("2330", Exchange::TSE);
    let tsmc_msg = format!("建立台積電股票合約：{}", tsmc.contract.base.code);
    log::info!("{}", tsmc_msg);
    println!("{}", tsmc_msg);
    
    // 台指期貨
    let taiex_future = client.create_future("TXFA4");
    let future_msg = format!("建立台指期貨合約：{}", taiex_future.contract.base.code);
    log::info!("{}", future_msg);
    println!("{}", future_msg);
    
    // 建立範例委託單（不會實際下單）
    log::info!("📝 建立範例委託單...");
    println!("\n📝 建立範例委託單...");
    
    let stock_order = rshioaji::Order::new(
        Action::Buy,
        500.0,      // 價格：新台幣500元
        1000,       // 數量：1張（1000股）
        OrderType::ROD,
        StockPriceType::LMT,
    );
    log::debug!("📦 股票委託單：{:?}", stock_order);
    println!("📦 股票委託單：{:?}", stock_order);
    
    let futures_order = rshioaji::FuturesOrder::new(
        Action::Buy,
        17000.0,    // 價格
        1,          // 數量：1口合約
        OrderType::ROD,
        rshioaji::FuturesPriceType::LMT,
        rshioaji::FuturesOCType::Auto,
    );
    log::debug!("🔮 期貨委託單：{:?}", futures_order);
    println!("🔮 期貨委託單：{:?}", futures_order);
    
    // 展示市場資料訂閱（需要登入才能使用）
    /*
    log::info!("📡 訂閱市場資料...");
    println!("\n📡 訂閱市場資料...");
    if let Err(e) = client.subscribe(tsmc.contract.clone(), QuoteType::Tick).await {
        log::warn!("⚠️  市場資料訂閱失敗：{}", e);
        println!("⚠️  市場資料訂閱失敗：{}", e);
    } else {
        log::info!("✅ 已訂閱台積電即時報價");
        println!("✅ 已訂閱台積電即時報價");
    }
    
    // 取得歷史資料
    log::info!("📊 取得歷史資料...");
    println!("\n📊 取得歷史資料...");
    let end_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let start_date = (chrono::Utc::now() - chrono::Duration::days(7))
        .format("%Y-%m-%d")
        .to_string();
    
    match client.kbars(tsmc.contract.clone(), &start_date, &end_date).await {
        Ok(kbars) => {
            let msg = format!("✅ 取得台積電 {} 筆 K 棒資料", kbars.data.len());
            log::info!("{}", msg);
            println!("{}", msg);
            if let Some(latest) = kbars.data.last() {
                let data_msg = format!(
                    "📈 最新台積電資料：開盤={}, 最高={}, 最低={}, 收盤={}, 成交量={}",
                    latest.open, latest.high, latest.low, latest.close, latest.volume
                );
                log::info!("{}", data_msg);
                println!("{}", data_msg);
            }
        }
        Err(e) => {
            log::error!("⚠️  無法取得 K 棒資料：{}", e);
            println!("⚠️  無法取得 K 棒資料：{}", e);
        }
    }
    
    // 登出
    log::info!("🚪 登出中...");
    println!("\n🚪 登出中...");
    let logout_success = client.logout().await?;
    if logout_success {
        log::info!("✅ 登出成功");
        println!("✅ 登出成功");
    } else {
        log::warn!("⚠️  登出可能失敗");
        println!("⚠️  登出可能失敗");
    }
    */
    
    log::info!("🎉 示範完成！");
    println!("\n🎉 示範完成！");
    println!("💡 要使用真實資料，請取消註解登入/登出區段並提供您的 API 憑證。");
    println!("\n📚 完整的登入流程說明：");
    println!("   1️⃣  token_login/simulation_login - 驗證憑證並建立連線");
    println!("   2️⃣  獲取帳戶清單和合約下載資訊");
    println!("   3️⃣  設定錯誤追蹤系統 (error_tracking)");
    println!("   4️⃣  下載合約資料 (當 fetch_contract=true)");
    println!("   5️⃣  設定預設股票帳戶 (stock_account)");
    println!("   6️⃣  設定預設期貨帳戶 (futopt_account)");
    println!("   7️⃣  準備就緒，可以開始交易");
    
    log::info!("範例執行完成，日誌已記錄到：{}", env_config.sj_log_path);
    
    Ok(())
}