use crate::errors::{CliCommandError, CliResult, Warning};
use crate::utility::macros::try_read_file_content;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "apply",
    about = "Apply a configuration(s). It will trigger update of all existing configurations, so it can take a while \
             due to rescaling of hardware and may require rerun of your applications"
)]
pub(crate) struct Apply {
    #[structopt(short, long)]
    config: Option<Vec<String>>,
}

impl Apply {
    fn apply(&self) -> CliResult<Option<Vec<Warning>>> {
        todo!()
    }
}
