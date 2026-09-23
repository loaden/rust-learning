use renamer as app;
use std::process;

fn main() {
    app::test_mod();

    let r = app::args::process_cmdline();
    match r {
        Ok(args) => println!("args: {:?}", args),
        Err(err) => println!("args: {}", err),
    }

    let _r = app::args::process_cmdline().unwrap_or_else(|err| {
        println!("{}", err);
        exit();
    });
}

fn exit() -> ! {
    process::exit(2)
}