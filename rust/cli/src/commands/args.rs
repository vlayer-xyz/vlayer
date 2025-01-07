use std::fmt;

use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, Parser)]
pub(crate) struct InitArgs {
    #[arg(long, value_enum)]
    pub(crate) template: Option<TemplateOption>,
    #[arg(long)]
    pub(crate) existing: bool,
    #[arg()]
    pub(crate) project_name: Option<String>,
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
