use std::env;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=package.json");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../.git");

    // 从 Cargo 环境变量获取信息
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.1.0".to_string());
    let name = env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "winterfox-cli".to_string());
    let description =
        env::var("CARGO_PKG_DESCRIPTION").unwrap_or_else(|_| "Winterfox CLI".to_string());
    let license = env::var("CARGO_PKG_LICENSE").unwrap_or_else(|_| "MIT".to_string());
    let authors = env::var("CARGO_PKG_AUTHORS").unwrap_or_else(|_| "Suyufox".to_string());
    let repository = env::var("CARGO_PKG_REPOSITORY").unwrap_or_else(|_| "".to_string());
    let homepage = env::var("CARGO_PKG_HOMEPAGE").unwrap_or_else(|_| "".to_string());

    // 获取构建时间戳
    let build_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // 获取 Git 信息
    let git_hash = get_git_hash().unwrap_or_else(|| "unknown".to_string());
    let git_branch = get_git_branch().unwrap_or_else(|| "unknown".to_string());

    // 获取构建环境信息
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let rustc = env::var("RUSTC").unwrap_or_else(|_| "unknown".to_string());

    // 创建构建信息文件
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("build_info.rs");

    let build_info = format!(
        r#"// 自动生成的构建信息文件
// 不要手动编辑此文件
// 此文件在构建时由 build.rs 生成

/// 包名称
pub const BUILD_NAME: &str = "{}";

/// 包描述
pub const BUILD_DESCRIPTION: &str = "{}";

/// 版本号
pub const BUILD_VERSION: &str = "{}";

/// 许可证
pub const BUILD_LICENSE: &str = "{}";

/// 作者
pub const BUILD_AUTHORS: &str = "{}";

/// 代码仓库
pub const BUILD_REPOSITORY: &str = "{}";

/// 主页
pub const BUILD_HOMEPAGE: &str = "{}";

/// 构建时间戳（UNIX 时间戳）
pub const BUILD_TIMESTAMP: u64 = {};

/// Git 提交哈希
pub const GIT_HASH: &str = "{}";

/// Git 分支
pub const GIT_BRANCH: &str = "{}";

/// 目标平台
pub const TARGET: &str = "{}";

/// 构建配置（debug/release）
pub const PROFILE: &str = "{}";

/// Rust 编译器
pub const RUSTC: &str = r"{}";

/// 完整的版本字符串
pub const FULL_VERSION: &str = "{} v{} ({} {})";

/// 获取构建信息摘要
pub fn build_info_summary() -> String {{
    format!(
        "{{}} v{{}} | {{}} | Build: {{}} | Git: {{}}",
        BUILD_NAME,
        BUILD_VERSION,
        TARGET,
        PROFILE,
        GIT_HASH
    )
}}

/// 获取详细的构建信息
pub fn build_info_detailed() -> String {{
    format!(
"Application: {{}} v{{}}
Description: {{}}
License: {{}}
Authors: {{}}
Repository: {{}}
Homepage: {{}}
Build Configuration: {{}}
Build Timestamp: {{}}
Git: {{}} on {{}}
Target Platform: {{}}
Rust Compiler: {{}}",
      BUILD_NAME,
      BUILD_VERSION,
      BUILD_DESCRIPTION,
      BUILD_LICENSE,
      BUILD_AUTHORS,
      BUILD_REPOSITORY,
      BUILD_HOMEPAGE,
      PROFILE,
      BUILD_TIMESTAMP,
      GIT_HASH,
      GIT_BRANCH,
      TARGET,
      RUSTC
    )
}}
"#,
        name,
        description,
        version,
        license,
        authors,
        repository,
        homepage,
        build_timestamp,
        git_hash,
        git_branch,
        target,
        profile,
        rustc,
        name,
        version,
        git_hash,
        git_branch
    );

    fs::write(&dest_path, build_info).expect("Failed to write build_info.rs");

    // 输出构建信息到控制台
    println!("cargo:warning=Building {} v{}", name, version);
    println!("cargo:warning=Target: {}, Profile: {}", target, profile);
    println!("cargo:warning=Git: {} on {}", git_hash, git_branch);
    println!("cargo:warning=Build timestamp: {}", build_timestamp);
}

fn get_git_hash() -> Option<String> {
    std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

fn get_git_branch() -> Option<String> {
    std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}
