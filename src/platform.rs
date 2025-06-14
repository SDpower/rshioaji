use std::path::PathBuf;

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
    pub fn so_extension(&self) -> Option<&'static str> {
        match self {
            Platform::MacOSArm => Some("cpython-312-darwin.so"),
            Platform::LinuxX86_64 => Some("cpython-312-x86_64-linux-gnu.so"),
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
        
        // Check for required Python files
        let required_files = [
            "__init__.py",
            "shioaji.py",
            "base.py",
            "constant.py",
            "contracts.py",
            "order.py",
        ];
        
        for file in &required_files {
            let file_path = shioaji_path.join(file);
            if !file_path.exists() {
                return Err(format!(
                    "Required Python file missing: {}",
                    file_path.display()
                ));
            }
        }
        
        // Check for backend directory and shared libraries
        let backend_path = shioaji_path.join("backend");
        if !backend_path.exists() {
            return Err(format!(
                "Backend directory not found: {}",
                backend_path.display()
            ));
        }
        
        let so_ext = self.so_extension()
            .ok_or_else(|| "Unknown shared library extension".to_string())?;
        
        let required_backend_libs = [
            format!("constant.{}", so_ext),
            format!("error.{}", so_ext),
            format!("utils.{}", so_ext),
        ];
        
        for lib in &required_backend_libs {
            let lib_path = backend_path.join(lib);
            if !lib_path.exists() {
                return Err(format!(
                    "Required backend library missing: {}",
                    lib_path.display()
                ));
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
        
        let required_solace_libs = [
            format!("api.{}", so_ext),
            format!("bidask.{}", so_ext),
            format!("quote.{}", so_ext),
            format!("tick.{}", so_ext),
            format!("utils.{}", so_ext),
        ];
        
        for lib in &required_solace_libs {
            let lib_path = solace_path.join(lib);
            if !lib_path.exists() {
                return Err(format!(
                    "Required solace library missing: {}",
                    lib_path.display()
                ));
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
                assert_eq!(platform.so_extension(), Some("cpython-312-darwin.so"));
            }
            Platform::LinuxX86_64 => {
                assert_eq!(platform.directory_name(), Some("manylinux_x86_64"));
                assert_eq!(platform.so_extension(), Some("cpython-312-x86_64-linux-gnu.so"));
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