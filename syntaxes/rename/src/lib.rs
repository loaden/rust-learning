pub mod io;
pub mod args;

use crate::io::*;

pub fn test_mod() {
    path::get_files(&"path".to_string());
    let _tf = types::TargetFile {
        new_name: String::new(),
        old_name: String::new(),
        renamed: false,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rename() {
        let cfg = args::Config::new(
            String::from("."),
            String::from("old"),
            String::from("new"),
        );

        let mut tf = types::TargetFile {
            new_name: String::new(),
            old_name: String::from("some_old_files.txt"),
            renamed: false,
        };
        let r = path::rename(&cfg, &mut tf);
        assert!(r.new_name.contains(&cfg.new_str));
    }
}
