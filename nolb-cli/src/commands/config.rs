use crate::errors::{CliCommandError, CliResult, Warning};
use crate::utility::macros::try_read_file_content;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "config", about = "Manage configurations")]
pub(crate) enum Config {
    Check(crate::commands::check::Check),
    Apply(crate::commands::apply::Apply),
    #[structopt(about = "Prints current config, which are currently in use by nolb")]
    Current,
}

impl Config {
    fn current(&self) -> &str {
        todo!()
    }
}
