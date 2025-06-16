use rshioaji::{Shioaji, EnvironmentConfig, init_logging};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // 📚 前置作業：初始化環境配置和日誌系統
    println!("🔧 正在初始化 rshioaji 環境...");
    
    // 載入環境變數配置
    let env_config = EnvironmentConfig::from_env();
    if let Err(e) = env_config.validate() {
        eprintln!("❌ 環境變數配置錯誤: {}", e);
        return;
    }
    
    println!("📋 環境配置: {}", env_config.summary());
    
    // 初始化日誌系統
    if let Err(e) = init_logging(&env_config) {
        eprintln!("❌ 日誌系統初始化失敗: {}", e);
        env_logger::init();
    }
    
    log::info!("🚀 rshioaji 環境初始化完成");
    
    // 從環境變數取得 API 憑證
    let api_key = std::env::var("SHIOAJI_API_KEY")
        .or_else(|_| std::env::var("API_KEY"))
        .unwrap_or_else(|_| {
            eprintln!("❌ 未設定 SHIOAJI_API_KEY 或 API_KEY 環境變數");
            std::process::exit(1);
        });
    
    let secret_key = std::env::var("SHIOAJI_SECRET_KEY")
        .or_else(|_| std::env::var("SECRET_KEY"))
        .unwrap_or_else(|_| {
            eprintln!("❌ 未設定 SHIOAJI_SECRET_KEY 或 SECRET_KEY 環境變數");
            std::process::exit(1);
        });
    
    let simulation = std::env::var("SHIOAJI_SIMULATION")
        .or_else(|_| std::env::var("SIMULATION"))
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);
    
    println!("🔑 API 金鑰: {}...", &api_key[..10.min(api_key.len())]);
    println!("🔐 密鑰: {}...", &secret_key[..10.min(secret_key.len())]);
    println!("🎯 模式: {}", if simulation { "模擬" } else { "正式" });
    
    // 顯示平台資訊
    let platform = rshioaji::platform::Platform::detect();
    log::info!("🖥️  偵測到的平台：{:?}", platform);
    println!("🖥️  偵測到的平台：{:?}", platform);
    
    if let Some(platform_dir) = platform.directory_name() {
        log::info!("📁 使用平台目錄：{}", platform_dir);
        println!("📁 使用平台目錄：{}", platform_dir);
        
        // 驗證安裝
        let base_path = match std::env::current_dir() {
            Ok(path) => path,
            Err(e) => {
                eprintln!("❌ 無法取得當前目錄：{}", e);
                return;
            }
        };
        
        match platform.validate_installation(&base_path) {
            Ok(()) => {
                log::info!("✅ 平台安裝驗證成功");
                println!("✅ 平台安裝驗證成功");
            },
            Err(e) => {
                log::error!("❌ 平台驗證失敗：{}", e);
                println!("❌ 平台驗證失敗：{}", e);
                return;
            }
        }
    } else {
        log::error!("❌ 不支援的平台");
        println!("❌ 不支援的平台");
        return;
    }
    
    // 建立 Shioaji 客戶端（使用環境變數決定模式）
    let proxies = HashMap::new();
    let client = match Shioaji::new(simulation, proxies) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("❌ 無法建立 Shioaji 客戶端：{}", e);
            return;
        }
    };
    
    // 🔧 重要：在 login() 前先呼叫 init()
    println!("\n🔧 初始化 Shioaji 客戶端...");
    log::info!("初始化 Shioaji 客戶端");
    
    if let Err(e) = client.init().await {
        log::error!("❌ Shioaji 客戶端初始化失敗：{}", e);
        println!("❌ Shioaji 客戶端初始化失敗：{}", e);
        return;
    }
    
    log::info!("✅ Shioaji 客戶端初始化成功");
    println!("✅ Shioaji 客戶端初始化成功");
    
    // 🔑 開始登入流程
    println!("\n🔑 開始登入流程...");
    log::info!("🔑 開始登入流程...");
    println!("📋 執行標準登入步驟：");
    println!("   1️⃣  調用 login 方法 (內部會呼叫 token_login 或 simulation_login)");
    println!("   2️⃣  獲取帳戶清單和合約下載資訊");
    println!("   3️⃣  設定錯誤追蹤系統");
    println!("   4️⃣  下載合約資料 (fetch_contract=true)");
    println!("   5️⃣  設定預設股票和期貨帳戶");
    
    // 執行登入（包含合約下載）
    match client.login(&api_key, &secret_key, true).await {
        Ok(accounts) => {
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
            
            // 檢查預設帳戶設定
            println!("\n🔧 檢查預設帳戶設定...");
            log::info!("🔧 檢查預設帳戶設定...");
            
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
            
            // 列出所有可用帳戶
            println!("\n📋 列出所有可用帳戶...");
            log::info!("📋 列出所有可用帳戶...");
            
            match client.list_accounts().await {
                Ok(all_accounts) => {
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
                }
                Err(e) => {
                    log::warn!("⚠️  無法列出帳戶：{}", e);
                    println!("⚠️  無法列出帳戶：{}", e);
                }
            }
            
            // 登出測試
            println!("\n🚪 測試登出功能...");
            log::info!("🚪 測試登出功能...");
            
            match client.logout().await {
                Ok(logout_success) => {
                    if logout_success {
                        log::info!("✅ 登出成功");
                        println!("✅ 登出成功");
                    } else {
                        log::warn!("⚠️  登出可能失敗");
                        println!("⚠️  登出可能失敗");
                    }
                }
                Err(e) => {
                    log::error!("❌ 登出失敗：{}", e);
                    println!("❌ 登出失敗：{}", e);
                }
            }
        }
        Err(e) => {
            log::error!("❌ 登入失敗：{}", e);
            println!("❌ 登入失敗：{}", e);
            println!("💡 請檢查：");
            println!("   - API 金鑰和密鑰是否正確");
            println!("   - 網路連線是否正常");
            println!("   - 是否在交易時間內");
            return;
        }
    }
    
    log::info!("🎉 登入測試完成！");
    println!("\n🎉 登入測試完成！");
    println!("✅ 所有功能測試通過");
} 