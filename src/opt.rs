use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "ss", about = "A simple CLI for the search command")]
pub struct Opt {
    #[structopt(subcommand)]
    pub cmd: Option<SubCommand>,

    #[structopt(short, default_value = "google")]
    pub use_website: String,
}

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    #[structopt(external_subcommand)]
    Content(Vec<String>),
}

