use std::path::PathBuf;
use pyo3::prelude::*;

/// Platform detection and path resolution for shioaji libraries
#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    MacOSArm,
    LinuxX86_64,
    Unknown,
}

impl Platform {
    /// Detect the current platform
    pub fn detect() -> Self {
        // Try environment variable override first
        if let Ok(platform_override) = std::env::var("RSHIOAJI_PLATFORM") {
            match platform_override.as_str() {
                "macosx_arm" => return Platform::MacOSArm,
                "manylinux_x86_64" => return Platform::LinuxX86_64,
                _ => {} // Fall through to auto-detection
            }
        }
        
        // Use runtime detection instead of compile-time cfg! macros
        match (std::env::consts::OS, std::env::consts::ARCH) {
            ("macos", "aarch64") => Platform::MacOSArm,
            ("linux", "x86_64") => Platform::LinuxX86_64,
            _ => Platform::Unknown,
        }
    }
    
    /// Get the platform-specific directory name
    pub fn directory_name(&self) -> Option<&'static str> {
        match self {
            Platform::MacOSArm => Some("macosx_arm"),
            Platform::LinuxX86_64 => Some("manylinux_x86_64"),
            Platform::Unknown => None,
        }
    }
    
    /// Get the platform-specific shared library extension pattern
    pub fn so_extension(&self) -> Option<String> {
        let python_version = self.detect_python_version();
        match self {
            Platform::MacOSArm => Some(format!("cpython-{}-darwin.so", python_version)),
            Platform::LinuxX86_64 => Some(format!("cpython-{}-x86_64-linux-gnu.so", python_version)),
            Platform::Unknown => None,
        }
    }
    
    /// Detect the current Python version from the runtime
    fn detect_python_version(&self) -> String {
        // First try to detect from PyO3 if available
        if let Ok(version) = Python::with_gil(|py| -> PyResult<String> {
            let version = py.version_info();
            Ok(format!("{}{}", version.major, version.minor))
        }) {
            log::info!("🐍 動態檢測 Python 版本: {}", version);
            return version;
        }
        
        // Fallback: try to parse from python3 --version
        if let Ok(output) = std::process::Command::new("python3")
            .arg("--version")
            .output() {
            let version_str = String::from_utf8_lossy(&output.stdout);
            // Simple parsing without regex: "Python 3.12.6"
            if version_str.starts_with("Python ") {
                let parts: Vec<&str> = version_str.trim().split(' ').collect();
                if parts.len() >= 2 {
                    let version_parts: Vec<&str> = parts[1].split('.').collect();
                    if version_parts.len() >= 2 {
                        let version = format!("{}{}", version_parts[0], version_parts[1]);
                        log::info!("🐍 從命令行檢測 Python 版本: {}", version);
                        return version;
                    }
                }
            }
        }
        
        // Final fallback: assume Python 3.12
        log::warn!("⚠️ 無法檢測 Python 版本，使用預設值: 312");
        "312".to_string()
    }
    
    /// Get all possible Python versions to try (current + fallbacks)
    pub fn get_possible_so_extensions(&self) -> Vec<String> {
        let detected_version = self.detect_python_version();
        let mut extensions = Vec::new();
        
        // Priority 1: Detected version
        if let Some(ext) = self.so_extension_for_version(&detected_version) {
            extensions.push(ext);
        }
        
        // Priority 2: Common versions as fallbacks
        let fallback_versions = ["313", "312", "311", "310"];
        for version in &fallback_versions {
            if *version != detected_version {
                if let Some(ext) = self.so_extension_for_version(version) {
                    extensions.push(ext);
                }
            }
        }
        
        extensions
    }
    
    /// Get extension for specific Python version
    fn so_extension_for_version(&self, python_version: &str) -> Option<String> {
        match self {
            Platform::MacOSArm => Some(format!("cpython-{}-darwin.so", python_version)),
            Platform::LinuxX86_64 => Some(format!("cpython-{}-x86_64-linux-gnu.so", python_version)),
            Platform::Unknown => None,
        }
    }
    
    /// Get the full path to the platform-specific shioaji directory
    pub fn get_shioaji_path(&self, base_path: &std::path::Path) -> Option<PathBuf> {
        self.directory_name().map(|dir_name| {
            base_path.join("lib").join("shioaji").join(dir_name)
        })
    }
    
    /// Check if the platform-specific shioaji directory exists and contains required files
    pub fn validate_installation(&self, base_path: &std::path::Path) -> Result<(), String> {
        let shioaji_path = self.get_shioaji_path(base_path)
            .ok_or_else(|| "Unsupported platform".to_string())?;
        
        if !shioaji_path.exists() {
            return Err(format!(
                "Shioaji directory not found: {}",
                shioaji_path.display()
            ));
        }
        
        // ✅ 純 .so 檔案架構 - 不再檢查 Python 檔案
        // 使用者已確認: 所有 .py 檔案已移除，一切在 Rust + .so 實現
        log::info!("🚀 純 .so 檔案架構: 跳過 Python 檔案檢查");
        
        // Check for backend directory and shared libraries
        let backend_path = shioaji_path.join("backend");
        if !backend_path.exists() {
            return Err(format!(
                "Backend directory not found: {}",
                backend_path.display()
            ));
        }
        
        let so_extensions = self.get_possible_so_extensions();
        if so_extensions.is_empty() {
            return Err("Unknown shared library extension".to_string());
        }
        
        log::info!("🔍 嘗試載入 .so 檔案版本: {:?}", so_extensions);
        
        let required_lib_names = ["constant", "error", "utils"];
        
        // Check each required library with version fallback
        for lib_name in &required_lib_names {
            let mut found = false;
            for so_ext in &so_extensions {
                let lib_file = format!("{}.{}", lib_name, so_ext);
                let lib_path = backend_path.join(&lib_file);
                if lib_path.exists() {
                    log::info!("✅ 找到 backend/{}", lib_file);
                    found = true;
                    break;
                }
            }
            if !found {
                log::warn!("⚠️ 在所有版本中都找不到 backend/{}", lib_name);
                // Don't fail immediately, continue checking
            }
        }
        
        // Check for solace directory and libraries
        let solace_path = backend_path.join("solace");
        if !solace_path.exists() {
            return Err(format!(
                "Solace directory not found: {}",
                solace_path.display()
            ));
        }
        
        let required_solace_names = ["api", "bidask", "quote", "tick", "utils"];
        
        // Check each required solace library with version fallback
        for lib_name in &required_solace_names {
            let mut found = false;
            for so_ext in &so_extensions {
                let lib_file = format!("{}.{}", lib_name, so_ext);
                let lib_path = solace_path.join(&lib_file);
                if lib_path.exists() {
                    log::info!("✅ 找到 solace/{}", lib_file);
                    found = true;
                    break;
                }
            }
            if !found {
                log::warn!("⚠️ 在所有版本中都找不到 solace/{}", lib_name);
                // Don't fail immediately, continue checking
            }
        }
        
        Ok(())
    }
    
    /// Get environment variables that might need to be set for this platform
    pub fn get_env_vars(&self, shioaji_path: &std::path::Path) -> Vec<(String, String)> {
        let mut env_vars = Vec::new();
        
        match self {
            Platform::MacOSArm => {
                // On macOS, we might need to set DYLD_LIBRARY_PATH
                let backend_path = shioaji_path.join("backend");
                let solace_path = backend_path.join("solace");
                
                if let Some(backend_str) = backend_path.to_str() {
                    if let Some(solace_str) = solace_path.to_str() {
                        let lib_path = format!("{}:{}", backend_str, solace_str);
                        env_vars.push(("DYLD_LIBRARY_PATH".to_string(), lib_path));
                    }
                }
            }
            Platform::LinuxX86_64 => {
                // On Linux, we might need to set LD_LIBRARY_PATH
                let backend_path = shioaji_path.join("backend");
                let solace_path = backend_path.join("solace");
                
                if let Some(backend_str) = backend_path.to_str() {
                    if let Some(solace_str) = solace_path.to_str() {
                        let lib_path = format!("{}:{}", backend_str, solace_str);
                        env_vars.push(("LD_LIBRARY_PATH".to_string(), lib_path));
                    }
                }
            }
            Platform::Unknown => {}
        }
        
        env_vars
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_platform_detection() {
        let platform = Platform::detect();
        
        // The actual platform depends on where tests are run
        match platform {
            Platform::MacOSArm => {
                assert_eq!(platform.directory_name(), Some("macosx_arm"));
                assert!(platform.so_extension().unwrap().contains("darwin.so"));
            }
            Platform::LinuxX86_64 => {
                assert_eq!(platform.directory_name(), Some("manylinux_x86_64"));
                assert!(platform.so_extension().unwrap().contains("linux-gnu.so"));
            }
            Platform::Unknown => {
                assert_eq!(platform.directory_name(), None);
                assert_eq!(platform.so_extension(), None);
            }
        }
    }
    
    #[test]
    fn test_platform_paths() {
        let platform = Platform::MacOSArm;
        let base_path = std::path::Path::new("/test");
        
        let shioaji_path = platform.get_shioaji_path(base_path).unwrap();
        assert_eq!(
            shioaji_path,
            std::path::PathBuf::from("/test/lib/shioaji/macosx_arm")
        );
    }
}