use clap::{Arg, ArgAction, Command, command};
use std::{env, path};
mod Rogblob;
mod Rogcommit;
mod init;
mod rogObject;
fn main() {
    let matches = command!()
        .subcommand(
            Command::new("init").about("create a new git repo").arg(
                Arg::new("path")
                    .default_value(".")
                    .help("where to create repo"),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", sub_m)) => {
            let path = sub_m.get_one::<String>("path").unwrap();

            init::GitRepo::init(path.into());
        }
        _ => println!("no cmd used"),
    };
}
