use std::env;
use std::path::Path;
use std::fs;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=lib/");
    
    // Detect the target platform
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    
    let platform_dir = match (target_os.as_str(), target_arch.as_str()) {
        ("macos", "aarch64") => "macosx_arm",
        ("linux", "x86_64") => "manylinux_x86_64",
        _ => {
            println!("cargo:warning=Unsupported platform: {}-{}", target_os, target_arch);
            return;
        }
    };
    
    println!("cargo:rustc-env=RSHIOAJI_PLATFORM={}", platform_dir);
    
    // Check if the platform-specific directory exists
    let lib_path = Path::new("lib/shioaji").join(platform_dir);
    if !lib_path.exists() {
        println!("cargo:warning=Platform-specific shioaji directory not found: {}", lib_path.display());
        println!("cargo:warning=Please ensure you have the correct shioaji libraries for {}", platform_dir);
        return;
    }
    
    // Check for required backend libraries
    let backend_path = lib_path.join("backend");
    if !backend_path.exists() {
        println!("cargo:warning=Backend directory not found: {}", backend_path.display());
        return;
    }
    
    let solace_path = backend_path.join("solace");
    if !solace_path.exists() {
        println!("cargo:warning=Solace directory not found: {}", solace_path.display());
        return;
    }
    
    // Determine the shared library extension based on platform
    let so_extension = match target_os.as_str() {
        "macos" => "cpython-312-darwin.so",
        "linux" => "cpython-312-x86_64-linux-gnu.so",
        _ => return,
    };
    
    // Check for required shared libraries
    let required_backend_libs = [
        format!("constant.{}", so_extension),
        format!("error.{}", so_extension),
        format!("utils.{}", so_extension),
    ];
    
    let required_solace_libs = [
        format!("api.{}", so_extension),
        format!("bidask.{}", so_extension),
        format!("quote.{}", so_extension),
        format!("tick.{}", so_extension),
        format!("utils.{}", so_extension),
    ];
    
    let mut missing_libs = Vec::new();
    
    for lib in &required_backend_libs {
        let lib_path = backend_path.join(lib);
        if !lib_path.exists() {
            missing_libs.push(format!("backend/{}", lib));
        }
    }
    
    for lib in &required_solace_libs {
        let lib_path = solace_path.join(lib);
        if !lib_path.exists() {
            missing_libs.push(format!("backend/solace/{}", lib));
        }
    }
    
    if !missing_libs.is_empty() {
        println!("cargo:warning=Missing required shared libraries for {}:", platform_dir);
        for lib in missing_libs {
            println!("cargo:warning=  - {}", lib);
        }
        println!("cargo:warning=Please ensure you have a complete shioaji installation");
    } else {
        println!("cargo:warning=Successfully validated shioaji installation for {}", platform_dir);
    }
    
    // Static linking configuration
    if env::var("CARGO_FEATURE_STATIC_LINK").is_ok() {
        embed_static_libraries(&backend_path, &solace_path, &target_os, so_extension);
    } else {
        // Set up library search paths for dynamic linking
        println!("cargo:rustc-link-search=native={}", backend_path.display());
        println!("cargo:rustc-link-search=native={}", solace_path.display());
        
        // For macOS, we might need additional library paths
        if target_os == "macos" {
            println!("cargo:rustc-env=DYLD_LIBRARY_PATH={0}:{1}", 
                     backend_path.display(), 
                     solace_path.display());
        }
        
        // For Linux, we might need additional library paths
        if target_os == "linux" {
            println!("cargo:rustc-env=LD_LIBRARY_PATH={0}:{1}", 
                     backend_path.display(), 
                     solace_path.display());
        }
    }
}

fn embed_static_libraries(backend_path: &Path, solace_path: &Path, target_os: &str, so_extension: &str) {
    println!("cargo:warning=Embedding shared libraries statically");
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let static_lib_dir = Path::new(&out_dir).join("static_libs");
    
    // Create static lib directory
    if !static_lib_dir.exists() {
        fs::create_dir_all(&static_lib_dir).expect("Failed to create static lib directory");
    }
    
    // Copy required libraries to static directory
    let required_backend_libs = [
        format!("constant.{}", so_extension),
        format!("error.{}", so_extension),
        format!("utils.{}", so_extension),
    ];
    
    let required_solace_libs = [
        format!("api.{}", so_extension),
        format!("bidask.{}", so_extension),
        format!("quote.{}", so_extension),
        format!("tick.{}", so_extension),
        format!("utils.{}", so_extension),
    ];
    
    // Copy backend libraries
    for lib in &required_backend_libs {
        let src = backend_path.join(lib);
        let dst = static_lib_dir.join(lib);
        if src.exists() {
            fs::copy(&src, &dst).unwrap_or_else(|_| panic!("Failed to copy {}", lib));
            println!("cargo:warning=Copied {} to static libs", lib);
        }
    }
    
    // Copy solace libraries
    for lib in &required_solace_libs {
        let src = solace_path.join(lib);
        let dst = static_lib_dir.join(lib);
        if src.exists() {
            fs::copy(&src, &dst).unwrap_or_else(|_| panic!("Failed to copy {}", lib));
            println!("cargo:warning=Copied {} to static libs", lib);
        }
    }
    
    // Generate static library archive
    create_static_archive(&static_lib_dir, target_os);
    
    // Link with the static archive
    println!("cargo:rustc-link-search=native={}", static_lib_dir.display());
    println!("cargo:rustc-link-lib=static=rshioaji_embedded");
}

fn create_static_archive(lib_dir: &Path, _target_os: &str) {
    let archive_name = "librshioaji_embedded.a";
    let archive_path = lib_dir.join(archive_name);
    
    // Get all .so files in the directory
    let so_files: Vec<_> = fs::read_dir(lib_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "so" {
                Some(path.file_name()?.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();
    
    if so_files.is_empty() {
        println!("cargo:warning=No .so files found to archive");
        return;
    }
    
    // Use ar to create static archive
    let ar_cmd = "ar";
    
    let mut cmd = Command::new(ar_cmd);
    cmd.arg("rcs").arg(&archive_path);
    
    for so_file in &so_files {
        cmd.arg(so_file);
    }
    
    let output = cmd.current_dir(lib_dir).output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                println!("cargo:warning=Created static archive: {} with {} files", 
                         archive_name, so_files.len());
                for file in &so_files {
                    println!("cargo:warning=  - {}", file);
                }
            } else {
                println!("cargo:warning=Failed to create static archive: {}", 
                         String::from_utf8_lossy(&result.stderr));
            }
        }
        Err(e) => {
            println!("cargo:warning=Error running ar command: {}", e);
        }
    }
}