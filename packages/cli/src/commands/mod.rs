use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum MainCommands {
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
