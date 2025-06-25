use rshioaji::{Shioaji, EnvironmentConfig, init_logging};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化環境和日誌
    println!("🔧 正在測試系統 shioaji 登入流程...");
    
    let env_config = EnvironmentConfig::from_env();
    if let Err(e) = env_config.validate() {
        eprintln!("❌ 環境變數配置錯誤: {}", e);
        return Ok(());
    }
    
    if let Err(e) = init_logging(&env_config) {
        eprintln!("❌ 日誌系統初始化失敗: {}", e);
        env_logger::init();
    }
    
    // 建立客戶端
    let proxies = HashMap::new();
    let client = Shioaji::new(true, proxies)?;  // 使用模擬模式測試
    
    // 初始化
    client.init().await?;
    log::info!("✅ 客戶端初始化成功");
    println!("✅ 客戶端初始化成功");
    
    // 測試登入（模擬模式）
    let test_api_key = "TEST_API_KEY";
    let test_secret_key = "TEST_SECRET_KEY";
    
    log::info!("🔑 開始登入測試...");
    println!("🔑 開始登入測試...");
    
    match client.login_simple(test_api_key, test_secret_key, true).await {
        Ok(accounts) => {
            log::info!("✅ 登入成功！獲得 {} 個帳戶", accounts.len());
            println!("✅ 登入成功！獲得 {} 個帳戶", accounts.len());
            
            for (i, account) in accounts.iter().enumerate() {
                let account_info = format!(
                    "📊 帳戶 {} - ID: {} ({}), 類型: {:?}, 已簽署: {}",
                    i + 1, account.account_id, account.username, account.account_type, account.signed
                );
                log::info!("{}", account_info);
                println!("{}", account_info);
            }
            
            // 測試帳戶功能
            if let Some(stock_account) = client.get_default_stock_account().await {
                println!("✅ 預設股票帳戶：{}", stock_account.account.account_id);
            }
            
            if let Some(future_account) = client.get_default_future_account().await {
                println!("✅ 預設期貨帳戶：{}", future_account.account.account_id);
            }
            
            // 測試登出
            println!("🚪 測試登出...");
            let logout_success = client.logout().await?;
            if logout_success {
                println!("✅ 登出成功");
            } else {
                println!("⚠️ 登出可能失敗");
            }
        }
        Err(e) => {
            log::error!("❌ 登入失敗：{}", e);
            println!("❌ 登入失敗：{}", e);
        }
    }
    
    println!("🎉 系統 shioaji 登入測試完成！");
    Ok(())
}