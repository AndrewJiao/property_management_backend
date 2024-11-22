use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 获取项目根目录（与 Cargo.toml 同级）
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    // 获取 Cargo 输出目录（构建时的目标目录）
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");

    // 定义配置文件路径
    let config_file = Path::new(&manifest_dir).join("config_dir.toml");

    // 定义目标路径（OUT_DIR 下的配置文件副本）
    let target_file = Path::new(&out_dir).join("config_dir.toml");

    // 复制配置文件到 OUT_DIR
    fs::copy(&config_file, &target_file).expect("Failed to copy config_dir.toml");

    // 输出给 Cargo，告诉它重新编译的条件
    println!("cargo:rerun-if-changed={}", config_file.display());
}
