use rshioaji::Shioaji;
use std::collections::HashMap;

/// 測試所有 8 個回調方法的基本 print 輸出
///
/// 此範例專門測試：
/// 1. 所有回調方法是否能正常註冊
/// 2. 佔位符 Python 函數是否有輸出
/// 3. 回調架構是否運作正常
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 測試所有 8 個回調方法的基本 print 輸出");
    println!("{}", "=".repeat(60));

    // 初始化環境
    dotenvy::dotenv().ok();
    env_logger::init();

    // 建立 Shioaji 客戶端（使用模擬模式）
    let proxies = HashMap::new();
    let client = Shioaji::new(true, proxies)?; // 模擬模式

    // 初始化客戶端
    client.init().await?;
    println!("✅ Shioaji 客戶端初始化成功");

    // 模擬登入（為了滿足回調註冊的前置條件）
    let api_key = std::env::var("SHIOAJI_API_KEY").unwrap_or_else(|_| "TEST_API_KEY".to_string());
    let secret_key =
        std::env::var("SHIOAJI_SECRET_KEY").unwrap_or_else(|_| "TEST_SECRET_KEY".to_string());

    println!("\n🔑 進行模擬登入...");
    match client.login_simple(&api_key, &secret_key, true).await {
        Ok(accounts) => {
            println!("✅ 登入成功，獲得 {} 個帳戶", accounts.len());
        }
        Err(e) => {
            println!("⚠️ 登入失敗（預期，使用測試憑證）：{}", e);
            println!("📋 繼續測試回調註冊...");
        }
    }

    println!("\n📞 開始註冊所有 8 個回調方法");
    println!("{}", "-".repeat(40));

    // 1. 註冊股票 tick 回調
    println!("\n1️⃣ 註冊 on_tick_stk_v1 回調...");
    match client
        .on_tick_stk_v1(
            |exchange, tick| {
                println!(
                    "📈 [Rust] 股票 Tick 收到: {:?} - {} @ {}",
                    exchange, tick.code, tick.close
                );
            },
            false,
        )
        .await
    {
        Ok(_) => println!("✅ on_tick_stk_v1 回調註冊成功"),
        Err(e) => println!("❌ on_tick_stk_v1 註冊失敗: {}", e),
    }

    // 2. 註冊期貨 tick 回調
    println!("\n2️⃣ 註冊 on_tick_fop_v1 回調...");
    match client
        .on_tick_fop_v1(
            |exchange, tick| {
                println!(
                    "📊 [Rust] 期貨 Tick 收到: {:?} - {} @ {}",
                    exchange, tick.code, tick.close
                );
            },
            false,
        )
        .await
    {
        Ok(_) => println!("✅ on_tick_fop_v1 回調註冊成功"),
        Err(e) => println!("❌ on_tick_fop_v1 註冊失敗: {}", e),
    }

    // 3. 註冊股票五檔回調
    println!("\n3️⃣ 註冊 on_bidask_stk_v1 回調...");
    match client
        .on_bidask_stk_v1(
            |exchange, bidask| {
                println!("💹 [Rust] 股票五檔收到: {:?} - {}", exchange, bidask.code);
            },
            false,
        )
        .await
    {
        Ok(_) => println!("✅ on_bidask_stk_v1 回調註冊成功"),
        Err(e) => println!("❌ on_bidask_stk_v1 註冊失敗: {}", e),
    }

    // 4. 註冊期貨五檔回調
    println!("\n4️⃣ 註冊 on_bidask_fop_v1 回調...");
    match client
        .on_bidask_fop_v1(
            |exchange, bidask| {
                println!("💰 [Rust] 期貨五檔收到: {:?} - {}", exchange, bidask.code);
            },
            false,
        )
        .await
    {
        Ok(_) => println!("✅ on_bidask_fop_v1 回調註冊成功"),
        Err(e) => println!("❌ on_bidask_fop_v1 註冊失敗: {}", e),
    }

    // 5. 註冊股票報價回調
    println!("\n5️⃣ 註冊 on_quote_stk_v1 回調...");
    match client
        .on_quote_stk_v1(
            |exchange, quote| {
                println!("📋 [Rust] 股票報價收到: {:?} - {}", exchange, quote.code);
            },
            false,
        )
        .await
    {
        Ok(_) => println!("✅ on_quote_stk_v1 回調註冊成功"),
        Err(e) => println!("❌ on_quote_stk_v1 註冊失敗: {}", e),
    }

    // 6. 註冊通用報價回調
    println!("\n6️⃣ 註冊 on_quote 回調...");
    match client
        .on_quote(|topic, data| {
            println!("📨 [Rust] 通用報價收到: {} - {:?}", topic, data);
        })
        .await
    {
        Ok(_) => println!("✅ on_quote 回調註冊成功"),
        Err(e) => println!("❌ on_quote 註冊失敗: {}", e),
    }

    // 7. 註冊系統事件回調
    println!("\n7️⃣ 註冊 on_event 回調...");
    match client
        .on_event(|resp_code, event_code, info, event| {
            println!(
                "⚡ [Rust] 系統事件收到: {} {} - {} {}",
                resp_code, event_code, info, event
            );
        })
        .await
    {
        Ok(_) => println!("✅ on_event 回調註冊成功"),
        Err(e) => println!("❌ on_event 註冊失敗: {}", e),
    }

    // 8. 註冊連線中斷回調
    println!("\n8️⃣ 註冊 on_session_down 回調...");
    match client
        .on_session_down(|| {
            println!("🚨 [Rust] 連線中斷通知！");
        })
        .await
    {
        Ok(_) => println!("✅ on_session_down 回調註冊成功"),
        Err(e) => println!("❌ on_session_down 註冊失敗: {}", e),
    }

    println!("\n{}", "=".repeat(60));
    println!("📊 回調註冊測試總結：");
    println!("✅ 所有 8 個回調方法註冊測試完成");
    println!("📋 檢查上方輸出，查看哪些回調成功註冊");

    println!("\n💡 注意事項：");
    println!("   • 目前是佔位符實作，不會有真實市場資料");
    println!("   • Python 佔位符函數應該會在註冊時輸出 print 訊息");
    println!("   • Rust 回調函數只有在收到真實資料時才會觸發");

    println!("\n🎯 下一步測試：");
    println!("   1. 檢查 Python 端的 print 輸出");
    println!("   2. 如果需要，可嘗試觸發模擬資料");
    println!("   3. 確認回調架構是否運作正常");

    // 短暫等待，讓可能的異步輸出有時間顯示
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    println!("\n✅ 回調測試完成！");

    Ok(())
}
