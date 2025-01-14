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
        let as_value = self
            .to_possible_value()
            .expect("no TemplateOption variant should be skipped");
        let name = as_value.get_name();
        write!(f, "{name}")
    }
}
