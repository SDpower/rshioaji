/// Platform detection for system compatibility
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

    /// Get a human-readable name for the platform
    pub fn name(&self) -> &'static str {
        match self {
            Platform::MacOSArm => "macOS ARM64",
            Platform::LinuxX86_64 => "Linux x86_64",
            Platform::Unknown => "Unknown",
        }
    }

    /// Check if the platform is supported for system shioaji integration
    pub fn is_supported(&self) -> bool {
        matches!(self, Platform::MacOSArm | Platform::LinuxX86_64)
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detection() {
        let platform = Platform::detect();

        // Test that we can detect some platform (won't be Unknown in CI)
        match std::env::consts::OS {
            "macos" if std::env::consts::ARCH == "aarch64" => {
                assert_eq!(platform, Platform::MacOSArm);
                assert!(platform.is_supported());
            }
            "linux" if std::env::consts::ARCH == "x86_64" => {
                assert_eq!(platform, Platform::LinuxX86_64);
                assert!(platform.is_supported());
            }
            _ => {
                assert_eq!(platform, Platform::Unknown);
                assert!(!platform.is_supported());
            }
        }
    }

    #[test]
    fn test_platform_names() {
        assert_eq!(Platform::MacOSArm.name(), "macOS ARM64");
        assert_eq!(Platform::LinuxX86_64.name(), "Linux x86_64");
        assert_eq!(Platform::Unknown.name(), "Unknown");
    }

    #[test]
    fn test_platform_display() {
        assert_eq!(format!("{}", Platform::MacOSArm), "macOS ARM64");
        assert_eq!(format!("{}", Platform::LinuxX86_64), "Linux x86_64");
        assert_eq!(format!("{}", Platform::Unknown), "Unknown");
    }
}
