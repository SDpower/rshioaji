use rshioaji::{
    Action, Exchange, FuturesOCType, FuturesOrder, FuturesPriceType, OrderType, Shioaji,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Creating Shioaji instance...");

    // Initialize environment
    dotenvy::dotenv().ok();
    env_logger::init();

    // 建立 Shioaji 客戶端（真實模式 - 符合純真實資料架構）
    let proxies = HashMap::new();
    let client = Shioaji::new(false, proxies)?;

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
    let accounts = client.login_simple(_api_key, _secret_key, true).await?;
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

    // 建立範例合約 - 使用 TAIFEX TXFG5 期貨（目前有資料行情）
    log::info!("📈 建立 TAIFEX TXFG5 期貨合約...");
    println!("\n📈 建立 TAIFEX TXFG5 期貨合約...");

    // TAIFEX TXFG5 期貨（台灣期貨交易所，目前有資料行情）
    let txfg5_future = client.create_future("TXFG5", Exchange::TAIFEX);
    let txfg5_msg = format!(
        "✅ 建立 TAIFEX TXFG5 期貨合約：{}",
        txfg5_future.contract.base.code
    );
    log::info!("{}", txfg5_msg);
    println!("{}", txfg5_msg);

    // 顯示合約資訊
    println!("📋 TXFG5 合約資訊：");
    println!("   🏷️  商品代碼: {}", txfg5_future.contract.base.code);
    println!("   🏛️  交易所: TAIFEX");
    println!("   📊 合約類型: 期貨");
    println!("   💹 目前有真實市場資料行情");

    // 建立 TXFG5 期貨委託單範例（不會實際下單）
    log::info!("📝 建立 TXFG5 期貨委託單範例...");
    println!("\n📝 建立 TXFG5 期貨委託單範例...");

    let txfg5_order = FuturesOrder::new(
        Action::Buy,
        17000.0, // 價格：17000點
        1,       // 數量：1口合約
        OrderType::ROD,
        FuturesPriceType::LMT,
        FuturesOCType::Auto,
    );
    log::debug!("🔮 TXFG5 期貨委託單：{:?}", txfg5_order);
    println!("🔮 TXFG5 期貨委託單：{:?}", txfg5_order);

    println!("📋 委託單詳細資訊：");
    println!("   📈 動作: 買進");
    println!("   💰 價格: 17000 點");
    println!("   📊 數量: 1 口");
    println!("   ⏰ 委託類型: ROD (當日有效)");
    println!("   🎯 價格類型: 限價單");

    // 展示 TXFG5 期貨市場資料訂閱（需要登入才能使用）
    /*
    log::info!("📡 訂閱 TXFG5 期貨市場資料...");
    println!("\n📡 訂閱 TXFG5 期貨市場資料...");
    if let Err(e) = client.subscribe(txfg5_future.contract.clone(), "tick").await {
        log::warn!("⚠️  TXFG5 期貨市場資料訂閱失敗：{}", e);
        println!("⚠️  TXFG5 期貨市場資料訂閱失敗：{}", e);
    } else {
        log::info!("✅ 已訂閱 TXFG5 期貨即時報價");
        println!("✅ 已訂閱 TXFG5 期貨即時報價");
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

    log::info!("範例執行完成");

    Ok(())
}
