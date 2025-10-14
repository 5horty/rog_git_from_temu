use configparser::ini;
use std::env;
use std::fs;
use std::path;

pub struct GitRepo {
    worktree: Option<path::PathBuf>,
    gitdir: Option<path::PathBuf>,
    conf: Option<ini::Ini>,
}
impl GitRepo {
    pub fn defualt() -> Self {
        Self {
            worktree: None,
            gitdir: None,
            conf: None,
        }
    }
    pub fn init() -> Self {
        let cwd = env::current_dir().expect("smt went wrong getting cwd");
        let gitdir = cwd.join(".git");
        println!("{cwd:?}");
        if gitdir.exists() {
            println!("already exists");
        } else {
            fs::create_dir_all(cwd.join(".git")).expect("failed to initailsed");
        }
        let mut conf = ini::Ini::new();
        let cf = gitdir.join("config");
        if cf.exists() {
            conf.load(&cf).expect("failed to load config");
        } else {
            conf.set("core", "repositoryformatversion", Some("0".to_owned()));
            conf.set("core", "filemode", Some("true".to_owned()));
            conf.set("core", "bare", Some("false".to_owned()));
            conf.write(&cf).expect("failed to write default config");
        }
        Self {
            worktree: Some(cwd),
            gitdir: Some(gitdir),
            conf: Some(conf),
        }
    }
    pub fn add() {
        todo!();
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn is() {
        todo!();
    }
}
