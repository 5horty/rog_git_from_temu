use clap::{Arg, ArgAction, Command, command};
use std::{env, path};
mod Rogblob;
mod Rogcommit;
mod init;
mod rogObject;
mod rogcat;
fn main() {
    let matches = command!()
        .subcommand(
            Command::new("init").about("create a new git repo").arg(
                Arg::new("path")
                    .default_value(".")
                    .help("where to create repo"),
            ),
        )
        .subcommand(
            Command::new("cat")
                .about("provide content")
                .arg(
                    Arg::new("type")
                        .help("specify the type")
                        .required(true)
                        .value_parser(["blob", "commit", "tag", "tree"]),
                )
                .arg(
                    Arg::new("object")
                        .help("the object to diplay")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", sub_m)) => {
            let path = sub_m.get_one::<String>("path").unwrap();

            init::GitRepo::init(path.into());
        }
        Some(("cat", sub_m)) => {
            let typ = sub_m.get_one::<String>("type").unwrap();
            let obj = sub_m.get_one::<String>("object").unwrap();
        }
        _ => println!("no cmd used"),
    };
}
