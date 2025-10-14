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
    pub fn repo_path(&self, components: &[&str]) -> Option<path::PathBuf> {
        let gitdir = self.gitdir.as_ref()?;
        let mut path = gitdir.clone();
        for comp in components {
            path = path.join(comp);
        }
        Some(path)
    }
    pub fn repo_dir(&self, components: &[&str], mkdir: bool) -> Option<path::PathBuf> {
        let gitdir = self.gitdir.as_ref()?;
        let mut path = gitdir.clone();
        for comp in components {
            path = path.join(comp);
        }
        if path.exists() {
            if path.is_dir() {
                return Some(path);
            } else {
                panic!("not a dir{path:?}");
            }
        }
        if mkdir {
            fs::create_dir_all(&path).ok()?;
        }
        Some(path)
    }

    pub fn repo_file(&self, components: &[&str], mkdir: bool) -> Option<path::PathBuf> {
        let parent = &components[..components.len() - 1];
        self.repo_dir(parent, mkdir)?;
        self.repo_path(components)
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
