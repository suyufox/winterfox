use clap::{Parser, Subcommand};

// 包含构建时生成的构建信息
// 这个文件在构建时由 build.rs 生成
include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

#[derive(Parser, Debug)]
#[command(
    name = BUILD_NAME,
    version = BUILD_VERSION,
    about = BUILD_DESCRIPTION,
    long_about = Some(BUILD_DESCRIPTION),
    author = BUILD_AUTHORS,
    help_template = "\
{before-help}{name} {version}
{author}
{about}

{usage-heading} {usage}

{all-args}{after-help}"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// 显示构建信息
    #[arg(short, long)]
    build_info: bool,

    /// 显示详细的构建信息
    #[arg(long)]
    build_info_detailed: bool,

    /// 显示版本信息
    #[arg(short, long)]
    version: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// 初始化配置文件
    Init {
        /// 配置文件路径
        #[arg(short, long, default_value = "winfox.config.yaml")]
        config: String,
    },

    /// 显示系统信息
    Info,

    /// 显示构建信息
    BuildInfo,
}

fn main() {
    let cli = Cli::parse();

    // 处理全局选项
    if cli.version {
        println!("{} v{}", BUILD_NAME, BUILD_VERSION);
        println!("Full version: {}", FULL_VERSION);
        return;
    }

    if cli.build_info {
        println!("{}", build_info_summary());
        return;
    }

    if cli.build_info_detailed {
        println!("{}", build_info_detailed());
        return;
    }

    // 处理子命令
    match cli.command {
        Some(Commands::Init { config }) => {
            println!("Initializing configuration at: {}", config);
            // 这里可以添加初始化配置文件的逻辑
        }
        Some(Commands::Info) => {
            show_system_info();
        }
        Some(Commands::BuildInfo) => {
            println!("=== Build Information ===");
            println!("{}", build_info_detailed());
        }
        None => {
            // 如果没有指定命令，显示帮助信息
            println!("Welcome to {} v{}", BUILD_NAME, BUILD_VERSION);
            println!("Type '{} --help' for more information", BUILD_NAME);
            println!();
            println!("Build: {}", build_info_summary());
        }
    }
}

fn show_system_info() {
    println!("=== System Information ===");
    println!("Application: {} v{}", BUILD_NAME, BUILD_VERSION);
    println!("Description: {}", BUILD_DESCRIPTION);
    println!("License: {}", BUILD_LICENSE);
    println!("Authors: {}", BUILD_AUTHORS);

    if !BUILD_REPOSITORY.is_empty() {
        println!("Repository: {}", BUILD_REPOSITORY);
    }

    if !BUILD_HOMEPAGE.is_empty() {
        println!("Homepage: {}", BUILD_HOMEPAGE);
    }

    println!();
    println!("=== Build Information ===");
    println!("Build Configuration: {}", PROFILE);
    println!("Build Timestamp: {}", BUILD_TIMESTAMP);
    println!("Git Commit: {}", GIT_HASH);
    println!("Git Branch: {}", GIT_BRANCH);
    println!("Target Platform: {}", TARGET);
    println!("Rust Compiler: {}", RUSTC);

    println!();
    println!("=== Environment ===");
    #[cfg(debug_build)]
    println!("This is a DEBUG build");

    #[cfg(release_build)]
    println!("This is a RELEASE build");

    #[cfg(has_build_info)]
    println!("Build info is available");
}
