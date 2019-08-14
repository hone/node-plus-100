use std::path::PathBuf;

use failure::Error;
use libbuildpack::Detect;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    #[allow(dead_code)]
    platform: PathBuf,
    #[structopt(parse(from_os_str))]
    #[allow(dead_code)]
    plan: PathBuf,
}

pub fn main() -> Result<(), Error> {
    let args = Cli::from_args();

    let detect = Detect::new(args.platform, args.plan)?;
    detect.pass(None)?;

    Ok(())
}
