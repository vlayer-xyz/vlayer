use alloy_primitives::ChainId;
use call_server::ProofMode;
use clap::{ArgAction, Parser, ValueEnum};
use std::fmt;

#[derive(Clone, Debug, Parser)]
pub(crate) struct InitArgs {
    #[arg(long, value_enum)]
    pub(crate) template: Option<TemplateOption>,
    #[arg(long)]
    pub(crate) existing: bool,
    #[arg()]
    pub(crate) project_name: Option<String>,
}

#[derive(Parser)]
pub(crate) struct ServeArgs {
    #[arg(long, action = ArgAction::Append, value_parser = parse_rpc_url)]
    pub(crate) rpc_url: Vec<(ChainId, String)>,

    #[arg(long, value_enum)]
    pub(crate) proof: Option<ProofModeArg>,
}

#[derive(Clone, Debug, ValueEnum, Default)]
pub(crate) enum TemplateOption {
    #[default]
    Simple,
    Airdrop,
    SimpleTravel,
    ERC20Balances,
    WebProof,
}

#[derive(Clone, Debug, ValueEnum, Default)]
pub(crate) enum ProofModeArg {
    #[default]
    Groth16,
    Fake,
}

impl ProofModeArg {
    pub(crate) fn map(self) -> ProofMode {
        match self {
            ProofModeArg::Groth16 => ProofMode::Groth16,
            ProofModeArg::Fake => ProofMode::Fake,
        }
    }
}

impl fmt::Display for TemplateOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TemplateOption::Simple => write!(f, "simple"),
            TemplateOption::Airdrop => write!(f, "airdrop"),
            TemplateOption::SimpleTravel => write!(f, "simple_travel"),
            TemplateOption::ERC20Balances => write!(f, "erc20_balances"),
            TemplateOption::WebProof => write!(f, "web_proof"),
        }
    }
}

fn parse_rpc_url(s: &str) -> Result<(ChainId, String), String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() < 2 {
        return Err("expected <chain-id>:<url>".to_string());
    }
    let chain_id: ChainId = parts[0]
        .parse()
        .map_err(|_| format!("Invalid chain ID: {}", parts[0]))?;
    let url = parts[1..].join(":");
    Ok((chain_id, url))
}
