use rshioaji::{Shioaji, SecurityType};
use std::collections::HashMap;
use std::sync::Arc;

/// 完整登入 API 示範 - 符合原始 shioaji.py 完整功能
/// 
/// 此範例展示如何使用完整的登入 API，包含所有原始 Python shioaji 支援的參數：
/// - contracts_timeout: 合約下載超時設定
/// - contracts_cb: 合約下載完成回調
/// - subscribe_trade: 是否訂閱交易事件
/// - receive_window: 接收視窗時間
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 完整登入 API 示範");
    println!("📋 符合原始 shioaji.py 的完整功能");
    
    // 初始化環境
    dotenvy::dotenv().ok();
    env_logger::init();
    
    // 建立 Shioaji 客戶端
    let proxies = HashMap::new();
    let client = Shioaji::new(true, proxies)?; // 使用模擬模式
    
    // 初始化客戶端
    client.init().await?;
    println!("✅ Shioaji 客戶端初始化成功");
    
    // 模擬 API 憑證（實際使用時請從環境變數獲取）
    let api_key = std::env::var("SHIOAJI_API_KEY").unwrap_or_else(|_| "demo_api_key".to_string());
    let secret_key = std::env::var("SHIOAJI_SECRET_KEY").unwrap_or_else(|_| "demo_secret_key".to_string());
    
    println!("\n🔑 使用完整登入 API...");
    println!("📊 參數說明：");
    println!("   • api_key: {}", &api_key[..4.min(api_key.len())]);
    println!("   • fetch_contract: true");
    println!("   • contracts_timeout: 30 秒");
    println!("   • contracts_cb: 有回調函數");
    println!("   • subscribe_trade: true");
    println!("   • receive_window: 30000ms");
    
    // 創建合約下載回調函數
    let contracts_callback = Arc::new(|security_type: SecurityType| {
        println!("📞 合約回調：{:?} 類型合約下載完成", security_type);
    });
    
    // 使用完整的登入 API
    let accounts = client.login(
        &api_key,
        &secret_key,
        true,                           // fetch_contract: 下載合約
        30,                             // contracts_timeout: 30 秒超時
        Some(Box::new(move |security_type| {
            println!("📞 合約下載完成：{:?}", security_type);
        })),                            // contracts_cb: 下載回調
        true,                           // subscribe_trade: 訂閱交易
        30000,                          // receive_window: 30 秒接收視窗
    ).await?;
    
    println!("✅ 完整登入成功！");
    println!("📊 獲得 {} 個帳戶", accounts.len());
    
    // 顯示帳戶資訊
    for (i, account) in accounts.iter().enumerate() {
        println!("   📋 帳戶 {}: {} ({}), 類型: {:?}, 已簽署: {}",
                i + 1, account.account_id, account.username, 
                account.account_type, account.signed);
    }
    
    // 檢查新增的功能
    println!("\n🔍 檢查新增功能：");
    
    // 檢查 person_id
    if let Some(person_id) = client.get_person_id().await {
        println!("   👤 Person ID: {}", person_id);
    }
    
    // 檢查錯誤追蹤
    if client.is_error_tracking_enabled().await {
        println!("   🔍 錯誤追蹤：已啟用");
    } else {
        println!("   🔍 錯誤追蹤：未啟用");
    }
    
    // 檢查 simulation-to-staging 模式
    if client.is_simu_to_stag() {
        println!("   🔄 模式：simulation-to-staging");
    } else {
        println!("   🔄 模式：標準模式");
    }
    
    // 檢查預設帳戶引用
    if let Some(stock_account) = client.get_default_stock_account_ref().await {
        println!("   🏦 預設股票帳戶引用：{}", stock_account.account.account_id);
    }
    
    if let Some(futopt_account) = client.get_default_futopt_account_ref().await {
        println!("   🔮 預設期貨帳戶引用：{}", futopt_account.account.account_id);
    }
    
    // 檢查合約資料
    if let Some(contracts) = client.get_contracts().await {
        println!("\n📋 合約資料檢查：");
        println!("   📊 狀態：{:?}", contracts.status);
        println!("   📈 股票合約：{}", contracts.counts.stocks);
        println!("   📊 期貨合約：{}", contracts.counts.futures);
        println!("   📉 選擇權合約：{}", contracts.counts.options);
        println!("   📈 指數合約：{}", contracts.counts.indices);
        println!("   🔢 總合約數：{}", contracts.total_count());
    }
    
    // 登出
    println!("\n🚪 登出中...");
    let logout_success = client.logout().await?;
    if logout_success {
        println!("✅ 登出成功");
    } else {
        println!("⚠️ 登出可能失敗");
    }
    
    println!("\n🎉 完整登入 API 示範完成！");
    println!("💡 這個範例展示了所有新增的登入功能，完全符合原始 Python shioaji API。");
    
    Ok(())
}