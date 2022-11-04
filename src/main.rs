use webbrowser;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "ss", about="A simple CLI for the search command")]
pub struct Opt {
    #[structopt(subcommand)]
    pub cmd: Option<SubCommand>,

    #[structopt(short, default_value="google")]
    pub use_website: String,
}

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    #[structopt(external_subcommand)]
    Other(Vec<String>),
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    match opt.cmd {
        Some(SubCommand::Other(args)) => {
            let mut url = String::from("https://");
            url.push_str(&opt.use_website);
            url.push_str(".com/search?q=");
            url.push_str(&args.join("+"));
            webbrowser::open(&url).unwrap();
        },
        None => {
            println!("No subcommand was used");
        }
    }; 
}
