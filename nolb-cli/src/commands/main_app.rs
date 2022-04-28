use structopt::StructOpt;

use super::config::Config;
use super::status::Status;

#[non_exhaustive]
#[derive(Debug, StructOpt)]
#[structopt(name = "nolb", about = "A console utility to manage your infrastructure")]
pub(crate) enum MainApp {
    Config(Config),
    Status(Status),
}
