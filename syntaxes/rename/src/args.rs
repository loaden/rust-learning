use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub path: PathBuf,
    pub old_str: String,
    pub new_str: String,
}

impl Config {
    pub fn new(path: String, old_str: String, new_str: String) -> Config {
        Config {
            path: PathBuf::from(path),
            old_str: old_str,
            new_str: new_str,
        }
    }
}

pub fn process_cmdline() -> Result<Config, &'static str> {
    let mut args = env::args();
    args.next();

    let path = match args.next() {
        Some(s) => s,
        None => return Err("path"),
    };
    let old_str = match args.next() {
        Some(s) => s,
        None => return Err("old_str"),
    };
    let new_str = match args.next() {
        Some(s) => s,
        None => return Err("old_str"),
    };

    let r = Config::new(
        path,
        old_str,
        new_str,
    );
    Ok(r)
}
