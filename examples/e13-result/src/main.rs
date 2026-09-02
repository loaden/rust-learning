use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, Write};

fn main() -> io::Result<()> {
    let filename = "test.txt";
    let mut file = match OpenOptions::new()
        .append(true)
        .read(true)
        .create(true)
        .open(filename)
    {
        Ok(file) => file,
        Err(_err) => panic!("what happened"),
    };

    file.write_all(b"test")?;
    file.rewind()?;

    let mut buf = String::new();
    match file.read_to_string(&mut buf) {
        Ok(n) => println!("f.read_to_string: {}, {}", buf, n),
        Err(e) => println!("{}", e),
    };

    match fs::read_to_string(filename) {
        Ok(s) => println!("read: {}", s),
        Err(e) => println!("read error: {}", e),
    }

    match test_read_file() {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Err: {}", e),
    }

    match test_report() {
        Ok(_f) => (),
        Err(e) => println!("Err: {}", e),
    }

    fs::remove_file(filename)?;
    Ok(())
}

fn test_read_file() -> Result<String, io::Error> {
    let mut f = File::open("non-exist")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn test_report() -> Result<File, io::Error> {
    let f = File::open("report.txt")?;
    Ok(f)
}
