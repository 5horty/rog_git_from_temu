use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct Repo {
    pub worktree: Option<PathBuf>,
    pub gitdir: Option<PathBuf>,
}
impl Repo {
    pub fn find_file(path: Option<&Path>) -> io::Result<Self> {
        let mut path = match path {
            Some(path) => path.to_path_buf(),
            None => env::current_dir()?,
        };
        loop {
            let gitdir = path.join(".git");
            if gitdir.is_dir() {
                return Ok(Self {
                    worktree: Some(path.clone()),
                    gitdir: Some(gitdir),
                });
            }
            if !path.pop() {
                break;
            }
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "not found git repo",
        ))
    }
}
