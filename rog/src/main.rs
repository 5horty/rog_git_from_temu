use clap::{Arg, ArgAction, Command, command};
use std::{env, path};
mod Rogblob;
mod Rogcommit;
mod init;
mod repo_find;
mod rogObject;
mod rogcat;
use Rogblob::RogBlob;
use init::GitRepo;
fn main() -> std::io::Result<()> {
    let repo = GitRepo::find_file(None)?;

    // Create a blob

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
                )
                .arg(
                    Arg::new("format")
                        .short('f')
                        .long("format")
                        .help("output format: t = type, p = print content")
                        .required(false)
                        .default_value("p")
                        .value_parser(["t", "p"]),
                ),
        )
        .subcommand(
            Command::new("blob").about("create a blob").arg(
                Arg::new("data")
                    .help("data to store in blob")
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
            let fmt = sub_m.get_one::<String>("format").unwrap();
            if let Err(e) = rogcat::cmd_cat_file(typ, obj, Some(fmt.as_str())) {
                println!("{}", e);
            }
        }
        Some(("blob", sub_m)) => {
            let repo = GitRepo::find_file(None)?;
            let data = sub_m.get_one::<String>("data").unwrap().as_bytes().to_vec();
            let blob = RogBlob::from_bytes(&data);
            let sha = rogObject::object_write(&repo, &blob)?;
            println!("Created blob with SHA: {}", sha);
        }
        _ => println!("no cmd used"),
    };
    Ok(())
}
