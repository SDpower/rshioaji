use rshioaji::{Action, Exchange, Order, OrderType, Shioaji, StockPriceType};
use std::collections::HashMap;

/// 合約存取範例 - 展示 get_system_contract 方法的正確用法
///
/// 此範例展示：
/// 1. 必須先登入才能存取合約
/// 2. 如何從已下載的合約資料中取得真實 Python 合約實例
/// 3. 錯誤處理和狀態檢查
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 合約存取範例");
    println!("🎯 展示 get_system_contract 方法的正確用法");

    // 初始化環境
    dotenvy::dotenv().ok();
    env_logger::init();

    // 建立 Shioaji 客戶端
    let proxies = HashMap::new();
    let client = Shioaji::new(true, proxies)?; // 使用模擬模式

    // 初始化客戶端
    client.init().await?;
    println!("✅ Shioaji 客戶端初始化成功");

    println!("\n❌ 示範：未登入時嘗試存取合約");

    // 創建一個測試用的股票合約
    let test_stock = client.create_stock("2330", Exchange::TSE);

    // 嘗試在未登入時下單（這會失敗）
    let test_order = Order::new(
        Action::Buy,
        500.0,
        1000,
        OrderType::ROD,
        StockPriceType::LMT,
    );

    match client
        .place_order(test_stock.contract.clone(), test_order)
        .await
    {
        Ok(_) => println!("   ⚠️ 未預期的成功（這不應該發生）"),
        Err(e) => {
            println!("   ✅ 正確行為：{}", e);
            println!("      📝 get_system_contract 方法會檢查登入狀態並拒絕存取");
        }
    }

    // 從環境變數獲取 API 憑證
    let api_key = std::env::var("SHIOAJI_API_KEY").unwrap_or_else(|_| "demo_api_key".to_string());
    let secret_key =
        std::env::var("SHIOAJI_SECRET_KEY").unwrap_or_else(|_| "demo_secret_key".to_string());

    println!("\n🔑 進行登入...");

    // 登入並下載合約資料
    let accounts = client
        .login(
            &api_key,
            &secret_key,
            true,  // fetch_contract: 下載合約資料
            30,    // contracts_timeout: 30 秒超時
            None,  // contracts_cb: 無回調
            false, // subscribe_trade: 不訂閱交易
            30000, // receive_window: 30 秒接收視窗
        )
        .await?;

    println!("✅ 登入成功！獲得 {} 個帳戶", accounts.len());

    println!("\n✅ 示範：登入後正確存取合約");

    // 現在可以正確存取合約
    let new_order = Order::new(
        Action::Buy,
        500.0,
        1000,
        OrderType::ROD,
        StockPriceType::LMT,
    );
    match client
        .place_order(test_stock.contract.clone(), new_order)
        .await
    {
        Ok(trade) => {
            println!("   ✅ 下單成功！");
            println!(
                "      📝 get_system_contract 現在能從 api.Contracts.Stocks[\"2330\"] 取得真實合約"
            );
            println!("      🆔 交易 ID: {}", trade.order_id);
            println!("      📊 狀態: {:?}", trade.status);
        }
        Err(e) => {
            println!("   ⚠️ 下單失敗：{}", e);
            println!("      📝 可能是模擬模式或其他交易限制");
        }
    }

    println!("\n📚 重要概念說明：");
    println!("   🔧 方法命名：get_system_contract (不是 create_system_contract)");
    println!("      • 語意正確：「取得」現有合約，而非「建立」新合約");
    println!("      • 架構對齊：與 Python shioaji 的 api.Contracts.Stocks[\"2330\"] 一致");

    println!("\n   🛡️ 安全檢查：必須先登入");
    println!("      • 防止未認證存取：get_system_contract 會檢查 logged_in 狀態");
    println!("      • 清楚錯誤訊息：指導使用者正確的呼叫順序");
    println!("      • 狀態驗證：確保系統處於正確狀態");

    println!("\n   📋 合約資料來源：");
    println!("      • 真實實例：從 logged-in instance 的 Contracts 屬性取得");
    println!("      • 類型導航：自動導航到 Stocks/Futures/Options/Indexs 集合");
    println!("      • 代碼查找：使用合約代碼 (如 \"2330\") 取得特定合約");

    // 登出
    client.logout().await?;
    println!("\n👋 登出成功");

    Ok(())
}
