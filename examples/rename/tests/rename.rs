use renamer as app;
use app::{
    args::Config,
    io::{
        types::TargetFile,
        path,
    }
};

#[test]
fn test_rename_integration() {
    let cfg = Config::new(
        String::from("."),
        String::from("old"),
        String::from("new")
    );
    let mut tf = TargetFile {
        new_name: String::new(),
        old_name: String::from("some_old_files.txt"),
        renamed: false,
    };
    let r = path::rename(&cfg, &mut tf);
    assert!(r.new_name.contains(&cfg.new_str));
    assert!(r.old_name.contains(&cfg.old_str));
    assert_eq!(r.renamed, false);
}
