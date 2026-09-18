use super::types;
use crate::args;

pub fn get_files(_path: &String) {
    let _t = types::TargetFile {
        old_name: String::from("old"),
        new_name: String::from("new"),
        renamed: false,
    };
}

pub fn rename<'a>(cfg: &args::Config, tf: &'a mut types::TargetFile) -> &'a types::TargetFile {
    println!("{:?}", cfg);
    tf.new_name = tf.old_name.replace(&cfg.old_str, &cfg.new_str);
    tf.renamed = true;
    tf
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_rename_inside() {
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

        let r = rename(&cfg, &mut tf);
        assert!(r.old_name.contains(&cfg.old_str));
        assert!(r.renamed);
    }
}
