use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum, Default, PartialEq, Eq, Copy)]
pub enum LogFormat {
    #[default]
    Plain,
    Json,
}

#[derive(Clone, Debug, Parser)]
pub struct GlobalArgs {
    /// A format for printing logs.
    #[arg(
        long,
        global = true,
        value_enum,
        env = "VLAYER_LOG_FORMAT",
        default_value = "plain"
    )]
    pub log_format: LogFormat,
}
