use clap_verbosity_flag::Verbosity;
use structopt::StructOpt;
use simplelog::{ Config, SimpleLogger };
use anyhow::Result;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(flatten)]
    verbose: Verbosity,
}

fn main() -> Result<()>{
    let opt = Opt::from_args();
    if let Some(level) = opt.verbose.log_level() {
        SimpleLogger::init(level.to_level_filter(), Config::default())?;
    }
    log::debug!("opt={:?}", opt);
    Ok(())
}
