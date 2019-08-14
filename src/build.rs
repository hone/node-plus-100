use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

use failure::Error;
use libbuildpack::Build;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    #[allow(dead_code)]
    layers: PathBuf,
    #[structopt(parse(from_os_str))]
    #[allow(dead_code)]
    platform: PathBuf,
    #[structopt(parse(from_os_str))]
    #[allow(dead_code)]
    plan: PathBuf,
}

pub fn main() -> Result<(), Error> {
    let args = Cli::from_args();

    println!("Constructing Build::new");
    let build = Build::new(args.layers, args.platform, args.plan)?;

    println!("Creating Middleware Layer");
    let mut middleware_layer = build.layers.add("middleware")?;
    println!("Setting launch = true");
    middleware_layer.config(|c| {
        c.launch = true;
    })?;

    println!("Writing middleware/index.js");
    let index_js = include_str!("../middleware/index.js");
    fs::write(middleware_layer.layer_path().join("index.js"), index_js)?;

    println!("Writing middleware/package.json");
    let package_json = include_str!("../middleware/package.json");
    fs::write(
        middleware_layer.layer_path().join("package.json"),
        package_json,
    )?;

    println!("Setting ENV VAR");
    middleware_layer
        .envs
        .launch
        .append_path
        .set_var("MIDDLEWARE_FUNCTION_URI", middleware_layer.layer_path());
    middleware_layer.write_envs()?;

    println!("Running npm install");
    let mut command = Command::new("npm")
        .args(&["install", "--production"])
        .current_dir(middleware_layer.layer_path())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let status = command.wait()?;

    println!("Status Code: {}", status.code().unwrap());

    Ok(())
}
