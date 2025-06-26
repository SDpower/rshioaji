fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // rshioaji now uses pure system shioaji integration
    // No longer requires embedded .so files or static linking
    println!("cargo:warning=rshioaji v0.4.7+ uses pure system shioaji integration");
    println!("cargo:warning=Please ensure system shioaji is installed: pip install shioaji");
}