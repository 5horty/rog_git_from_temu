use crate::init::{self, GitRepo};
use crate::rogObject::object_read;
use std::io;

pub fn cmd_cat_file(obj_type: &str, obj_name: &str) -> io::Result<()> {
    let repo = GitRepo::find_file(None)?;
    cat_file(&repo, obj_name, Some("p"))
}

pub fn cat_file(repo: &GitRepo, obj_name: &str, fmt: Option<&str>) -> io::Result<()> {
    //let gitdir = repo
    //.gitdir
    //.as_ref()
    //.ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "repo has no gitdir"))?;
    let obj = object_read(repo, obj_name).unwrap();
    match fmt.unwrap_or("p") {
        "t" => println!("{}", obj.fmt()),
        "p" => {
            let data = obj.serialize();
            println!("{}", String::from_utf8_lossy(&data));
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid format",
            ));
        }
    }
    Ok(())
}
