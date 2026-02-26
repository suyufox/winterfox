use clap::Parser;

// 包含构建时生成的构建信息
// 这个文件在构建时由 build.rs 生成
include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

mod commands;

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
struct Winfox {
    #[command(subcommand)]
    command: Option<commands::MainCommands>,

    /// 显示构建信息
    #[arg(long)]
    build_info: bool,

    /// 显示详细的构建信息
    #[arg(long)]
    build_info_detailed: bool,

    /// 显示版本信息
    #[arg(short, long)]
    version: bool,
}

fn main() {
    let cli = Winfox::parse();

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

    use commands::MainCommands;

    match cli.command {
        Some(MainCommands::Init { config }) => {
            println!("Initializing configuration at: {}", config);
            // 这里可以添加初始化配置文件的逻辑
        }
        Some(MainCommands::Info) => {
            show_system_info();
        }
        Some(MainCommands::BuildInfo) => {
            println!("=== Build Information ===");
            println!("{}", build_info_detailed());
        }
        None => {}
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
    #[cfg(debug_assertions)]
    println!("This is a DEBUG build");

    #[cfg(not(debug_assertions))]
    println!("This is a RELEASE build");
}
