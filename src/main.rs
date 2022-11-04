use anyhow::{anyhow, Context, Ok, Result};
use opt::SubCommand::Content;
use structopt::StructOpt;
use webbrowser;

mod config;
mod opt;

#[tokio::main]
async fn main() -> Result<()> {
    let opt = opt::Opt::from_args();
    let mut config = config::init_config();
    let read_config = config::read_config().await?;
    read_config.iter().for_each(|item| {
        config.insert(item.0.clone(), item.1.clone());
    });
    let mut keys = config.keys();
    match opt.cmd {
        Some(Content(arg)) => {
            let key = &opt.use_website;
            let resutl = keys.find(|x| *x == key);
            if resutl.is_none() {
                // key = config.get("default").unwrap();
                return Err(anyhow!("error website: {} config", key));
            } else {
                let url = config
                    .get(key.as_str())
                    .with_context(|| format!("{} is not a valid website", opt.use_website))?;
                let url = url.replace("{keyword}", &arg.join("+"));
                println!("{}", url);
                webbrowser::open(&url).unwrap();
            } // println!("key: {}", key);
        }
        None => {
            println!("No subcommand was used");
        }
    };
    Ok(())
}
