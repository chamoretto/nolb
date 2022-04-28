use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "status",
    about = "Display current state of the network, including all applied configurations."
)]
pub(crate) struct Status {
    #[structopt(short, long)]
    colored: bool,
}
