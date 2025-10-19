use crate::init;
use crate::repo_find::Repo;
use std::io;

pub fn cmd_cat_file(obj_type: &str, obj_name: &str) -> io::Result<()> {
    let repo = Repo::find_file(None)?;
}

pub fn cat_file(repo: &Repo, obj_name: &str, fmt: Option<&str>) -> io::Result<()> {
    let obj_id = 
}
