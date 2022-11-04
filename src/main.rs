use anyhow::{Context, Ok, Result};
use opt::SubCommand::Content;
use structopt::StructOpt;
use webbrowser;

mod opt;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
    config::read_config().await?;
    let opt = opt::Opt::from_args();
    let config = config::init_config();
    match opt.cmd {
        Some(Content(arg)) => {
            let url = config
                .get(&opt.use_website[..])
                .with_context(|| format!("{} is not a valid website", opt.use_website))?;
            let url = url.replace("{keyword}", &arg.join("+"));
            webbrowser::open(&url).unwrap();
        }
        None => {
            println!("No subcommand was used");
        }
    };
    Ok(())
}

