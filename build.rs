use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 获取项目根目录（与 Cargo.toml 同级）
    let manifest_dir = format!("{}/..", env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    println!("manifest_dir: {}", manifest_dir);

    // 获取 Cargo 输出目录（构建时的目标目录）
    let out_dir = format!("{}/../../../", env::var("OUT_DIR").expect("OUT_DIR not set"));

    // 定义配置文件路径
    let config_dir = Path::new(&manifest_dir).join("config_dir");
    if !config_dir.exists() {
        panic!("Config directory does not exist: {}", config_dir.display());
    }

    //创建新的文件夹
    println!("Creating output directory: {}", out_dir);
    let config_dir_path = format!("{}/config_dir", &out_dir);
    if fs::exists(&config_dir_path).unwrap() {
        println!("config_dir exists");
    } else {
        println!("create new config_dir");
        fs::create_dir(&config_dir_path).expect("Failed to create output directory");
    }

    // 遍历 config 目录下的文件并复制到 OUT_DIR
    for entry in fs::read_dir(&config_dir).expect("Failed to read config directory") {
        let entry = entry.expect("Failed to read entry");
        let src_path = entry.path();
        if src_path.is_file() {
            let file_name = src_path.file_name().expect("Failed to get file name");
            println!("Copying file: {:?}", file_name);
            let dest_path = Path::new(&out_dir).join("config_dir").join(file_name);
            println!("Copying to: {:?}", dest_path);
            fs::copy(&src_path, &dest_path)
                .expect(&format!("Failed to copy file: {:?}", src_path));
        }
    }

    // 通知 Cargo，如果配置文件改变，需要重新运行构建脚本
    println!("cargo:rerun-if-changed=config/");
}
