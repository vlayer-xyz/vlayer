use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, ValueEnum, Default, PartialEq, Eq, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
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
