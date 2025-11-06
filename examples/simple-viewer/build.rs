fn main() {
    // Load .env.local from project root (two levels up from example directory)
    let env_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join(".env.local"));

    if let Some(path) = env_path {
        if path.exists() {
            // Read the file and parse CESIUM_ION_TOKEN
            if let Ok(contents) = std::fs::read_to_string(&path) {
                for line in contents.lines() {
                    if let Some(token) = line.strip_prefix("CESIUM_ION_TOKEN=") {
                        // Remove quotes if present
                        let token = token.trim().trim_matches('"');
                        println!("cargo:rustc-env=CESIUM_ION_TOKEN={}", token);
                        println!("cargo:rerun-if-changed={}", path.display());
                        return;
                    }
                }
            }
        }
    }

    // If no token found, print warning
    println!(
        "cargo:warning=CESIUM_ION_TOKEN not found in .env.local - viewer will use default token"
    );
}
