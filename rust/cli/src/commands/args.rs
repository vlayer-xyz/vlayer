use std::fmt;

use alloy_primitives::ChainId;
use call_guest_wrapper::GUEST_ELF as CALL_GUEST_ELF;
use call_server::{Config, ProofMode};
use chain_guest_wrapper::GUEST_ELF as CHAIN_GUEST_ELF;
use clap::{ArgAction, Parser, ValueEnum};

#[derive(Clone, Debug, Parser)]
pub(crate) struct InitArgs {
    #[arg(long, value_enum)]
    pub(crate) template: Option<TemplateOption>,
    #[arg(long)]
    pub(crate) existing: bool,
    #[arg()]
    pub(crate) project_name: Option<String>,
}

#[derive(Parser, Default, Debug)]
pub(crate) struct ServeArgs {
    #[arg(long, action = ArgAction::Append, value_parser = parse_rpc_url)]
    pub(crate) rpc_url: Vec<(ChainId, String)>,

    #[arg(long, value_enum)]
    pub(crate) proof: Option<ProofModeArg>,

    /// Host to listen on.
    #[arg(long, default_value = "127.0.0.1")]
    pub(crate) host: Option<String>,

    /// Port to listen on.
    #[arg(long, short, default_value = "3000")]
    pub(crate) port: Option<u16>,

    #[arg(long)]
    pub(crate) verify_chain_proofs: bool,
}

impl ServeArgs {
    pub fn into_server_config(self, chain_proof_url: impl ToString, api_version: String) -> Config {
        let proof_mode = self.proof.unwrap_or_default().map();
        call_server::ConfigBuilder::new(
            chain_proof_url,
            CALL_GUEST_ELF.clone(),
            CHAIN_GUEST_ELF.clone(),
            api_version,
        )
        .with_rpc_mappings(self.rpc_url)
        .with_proof_mode(proof_mode)
        .with_host(self.host)
        .with_port(self.port)
        .with_verify_chain_proofs(self.verify_chain_proofs)
        .build()
    }
}

#[derive(Clone, Debug, ValueEnum, Default)]
pub(crate) enum TemplateOption {
    #[default]
    Simple,
    SimpleEmailProof,
    SimpleTeleport,
    SimpleTimeTravel,
    SimpleWebProof,
}

#[derive(Clone, Debug, ValueEnum, Default, PartialEq, Eq)]
pub(crate) enum ProofModeArg {
    #[default]
    Fake,
    Groth16,
}

impl ProofModeArg {
    pub(crate) const fn map(self) -> ProofMode {
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
            TemplateOption::SimpleEmailProof => write!(f, "simple_email_proof"),
            TemplateOption::SimpleTeleport => write!(f, "simple_teleport"),
            TemplateOption::SimpleTimeTravel => write!(f, "simple_time_travel"),
            TemplateOption::SimpleWebProof => write!(f, "simple_web_proof"),
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

#[cfg(test)]
mod tests {
    use super::*;

    mod serve_args {
        use super::*;

        #[test]
        fn default_proving_mode_is_fake() {
            let args: ServeArgs = Default::default();
            assert_eq!(args.proof.unwrap_or_default(), ProofModeArg::Fake);
        }
    }
}
