use clap::{ArgAction, Parser, ValueEnum};
use std::fmt;

#[derive(Clone, Debug, Parser)]
pub(crate) struct InitArgs {
    #[arg(long, value_enum)]
    pub(crate) template: Option<TemplateOption>,
}

#[derive(Parser)]
pub(crate) struct ServeArgs {
    #[arg(long, action = ArgAction::Append, value_parser = parse_rpc_url)]
    pub(crate) rpc_url: Vec<(u64, String)>,
}

#[derive(Clone, Debug, ValueEnum, Default)]
pub(crate) enum TemplateOption {
    #[default]
    Simple,
    Airdrop,
    SimpleTravel,
    ERC20Balances,
}

impl fmt::Display for TemplateOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TemplateOption::Simple => write!(f, "simple"),
            TemplateOption::Airdrop => write!(f, "airdrop"),
            TemplateOption::SimpleTravel => write!(f, "simple_travel"),
            TemplateOption::ERC20Balances => write!(f, "erc20_balances"),
        }
    }
}

fn parse_rpc_url(s: &str) -> Result<(u64, String), String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() < 2 {
        return Err(format!("expected <chain-id>:<url>"));
    }
    let chain_id: u64 = parts[0]
        .parse()
        .map_err(|_| format!("Invalid chain ID: {}", parts[0]))?;
    let url = parts[1..].join(":");
    Ok((chain_id, url))
}
