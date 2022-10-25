use std::env;
use std::path::{PathBuf, Path};

fn main()
{
    let settings_file_name = "settings.json";
    let settings_file = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join(settings_file_name);
    println!("cargo:rerun-if-changed={}", settings_file.to_str().unwrap());
    
    let target_dir = get_target_dir();
    println!("cargo:info=FileToCopy: {}", settings_file.to_str().unwrap());
    println!("cargo:info=TargetDir: {}", target_dir.to_str().unwrap());
    
    let target_file = target_dir.join(settings_file_name);
    if let Ok(bytes) = std::fs::copy(settings_file, target_file) {
        println!("cargo:info=BytesCopied: {}", bytes);
    } else {
        println!("cargo:error=Failed to copy file");
    }
}

fn get_target_dir() -> PathBuf
{
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_profile = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir).join("target").join(build_profile);
    PathBuf::from(path)
}