use clap::{Parser, ValueEnum};
use std::fmt;

#[derive(Clone, Debug, Parser)]
pub(crate) struct InitArgs {
    #[arg(long, value_enum)]
    pub(crate) template: Option<TemplateOption>,
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
