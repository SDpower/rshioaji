use rshioaji::Shioaji;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// 專門測試回調機制是否正常工作
///
/// 此範例會：
/// 1. 註冊回調函數
/// 2. 登入（可能觸發事件）
/// 3. 驗證回調函數能否被觸發
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 回調機制測試");
    println!("{}", "=".repeat(50));

    // 初始化環境
    dotenvy::dotenv().ok();
    env_logger::init();

    // 計數器
    let event_count = Arc::new(AtomicUsize::new(0));

    // 建立 Shioaji 客戶端
    let proxies = HashMap::new();
    let client = Shioaji::new(false, proxies)?; // 使用真實模式

    // 初始化客戶端
    client.init().await?;
    println!("✅ Shioaji 客戶端初始化成功");

    // 註冊系統事件回調
    let counter = event_count.clone();
    client
        .on_event(move |resp_code, event_code, info, event| {
            let count = counter.fetch_add(1, Ordering::SeqCst) + 1;
            println!(
                "🎯 [回調機制測試] 事件 #{:03}: 代碼 {} {} | {} | {}",
                count, resp_code, event_code, info, event
            );
        })
        .await?;
    println!("✅ 系統事件回調註冊成功");

    // 取得憑證
    let api_key = std::env::var("SHIOAJI_API_KEY")
        .map_err(|_| "Please set SHIOAJI_API_KEY environment variable")?;
    let secret_key = std::env::var("SHIOAJI_SECRET_KEY")
        .map_err(|_| "Please set SHIOAJI_SECRET_KEY environment variable")?;

    println!("\n🔑 開始登入測試...");
    println!("   API Key: {}****", &api_key[..4.min(api_key.len())]);

    // 登入
    match client.login_simple(&api_key, &secret_key, true).await {
        Ok(accounts) => {
            println!("✅ 登入成功，獲得 {} 個帳戶", accounts.len());

            // 等待一段時間看是否有事件觸發
            println!("\n⏳ 等待 3 秒，觀察是否有事件觸發...");
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

            let final_count = event_count.load(Ordering::SeqCst);
            println!("\n📊 回調測試結果：");
            println!("   觸發的事件總數: {} 次", final_count);

            if final_count > 0 {
                println!("🎉 回調機制正常工作！");
            } else {
                println!("⚠️ 沒有事件被觸發，可能原因：");
                println!("   • 沒有系統事件發生");
                println!("   • 回調轉發機制需要進一步調試");
            }

            // 登出
            client.logout().await?;
            println!("✅ 登出成功");
        }
        Err(e) => {
            println!("❌ 登入失敗: {}", e);
        }
    }

    println!("\n🏁 回調機制測試完成");
    Ok(())
}
