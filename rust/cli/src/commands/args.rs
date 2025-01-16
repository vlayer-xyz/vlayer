use std::{fmt, path::PathBuf};

use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, Parser)]
pub(crate) struct InitArgs {
    /// Template to use for the project
    #[arg(long, value_enum)]
    pub(crate) template: Option<TemplateOption>,
    /// Force init in existing project location
    #[arg(long)]
    pub(crate) existing: bool,
    /// Name of the project
    #[arg()]
    pub(crate) project_name: Option<String>,
    /// Directory where the templates will be unpacked into (useful for debugging)
    #[arg(long, env = "VLAYER_WORK_DIR")]
    pub(crate) work_dir: Option<PathBuf>,
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
