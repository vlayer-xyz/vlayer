use clap::{Arg, ArgAction, ArgMatches, Args, FromArgMatches, Parser, Subcommand};
use std::ffi::OsString;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

pub struct TestArgs {
    pub args: Vec<OsString>,
}

impl Args for TestArgs {
    fn augment_args(cmd: clap::Command) -> clap::Command {
        cmd.disable_help_flag(true).arg(
            Arg::new("args")
                .action(ArgAction::Append)
                .allow_hyphen_values(true)
                .trailing_var_arg(true),
        )
    }

    fn augment_args_for_update(cmd: clap::Command) -> clap::Command {
        Self::augment_args(cmd)
    }
}

impl FromArgMatches for TestArgs {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, clap::Error> {
        let args = matches
            .get_raw("args")
            .unwrap_or_default()
            .map(|i| i.to_owned())
            .collect();

        Ok(Self { args })
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), clap::Error> {
        *self = Self::from_arg_matches(matches)?;

        Ok(())
    }
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Serve,
    Test(TestArgs),
}
