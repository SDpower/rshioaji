use rshioaji::platform::Platform;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();
    
    // Show platform information
    let platform = Platform::detect();
    println!("🖥️  Detected platform: {:?}", platform);
    
    if let Some(platform_dir) = platform.directory_name() {
        println!("📁 Using platform directory: {}", platform_dir);
        
        // Validate installation
        let base_path = std::env::current_dir()?;
        match platform.validate_installation(&base_path) {
            Ok(()) => {
                println!("✅ Platform installation validated successfully");
                
                // Get the shioaji path
                let shioaji_path = platform.get_shioaji_path(&base_path).unwrap();
                println!("📂 Shioaji path: {}", shioaji_path.display());
                
                // List files in the directory
                if let Ok(entries) = std::fs::read_dir(&shioaji_path) {
                    println!("📋 Files in shioaji directory:");
                    for entry in entries {
                        if let Ok(entry) = entry {
                            println!("  - {}", entry.file_name().to_string_lossy());
                        }
                    }
                }
                
                // Check backend directory
                let backend_path = shioaji_path.join("backend");
                if backend_path.exists() {
                    println!("✅ Backend directory exists");
                    
                    if let Ok(entries) = std::fs::read_dir(&backend_path) {
                        println!("📋 Files in backend directory:");
                        for entry in entries {
                            if let Ok(entry) = entry {
                                let file_name = entry.file_name().to_string_lossy().to_string();
                                if file_name.ends_with(".so") {
                                    println!("  ✅ {}", file_name);
                                } else {
                                    println!("  - {}", file_name);
                                }
                            }
                        }
                    }
                }
                
                // Check solace directory
                let solace_path = backend_path.join("solace");
                if solace_path.exists() {
                    println!("✅ Solace directory exists");
                    
                    if let Ok(entries) = std::fs::read_dir(&solace_path) {
                        println!("📋 Files in solace directory:");
                        for entry in entries {
                            if let Ok(entry) = entry {
                                let file_name = entry.file_name().to_string_lossy().to_string();
                                if file_name.ends_with(".so") {
                                    println!("  ✅ {}", file_name);
                                } else {
                                    println!("  - {}", file_name);
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("❌ Platform validation failed: {}", e);
                println!("💡 Please ensure you have the correct shioaji libraries for your platform");
            }
        }
    } else {
        println!("❌ Unsupported platform");
    }
    
    println!("\n🎉 Platform validation completed!");
    
    Ok(())
}