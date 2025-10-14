use clap::{Arg, ArgAction, Command, command};
mod init;
fn main() {
    let cmd = command!().arg(Arg::new("name").short('n'));
    let matches = cmd.get_matches();

    if let Some(name) = matches.get_one::<String>("name") {
        println!("hello from args ur name is {name}");
    }
    let i = init::GitRepo::init();
}
