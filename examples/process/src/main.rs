use std::io;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

fn main() -> io::Result<()> {
    if let Ok(mut child) = Command::new("clang")
        .arg("-v")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        let mut stderr = child.stderr.take().unwrap();
        let mut str = String::new();
        stderr.read_to_string(&mut str)?;
        println!("ERROR: {:#?}", str);
        let v: Vec<_> = str.split("InstalledDir:").collect();
        println!("Target: {:#?}", v[1]);
        let mut path = PathBuf::from(v[1].trim());
        path.pop();
        if let Some(s) = path.to_str() {
            println!("DONE: {:?}", s);
        }

        let output = child.wait_with_output()?;
        if output.status.success() {
            let raw_output = String::from_utf8(output.stdout).unwrap();
            println!("OUTPUT: {:#?}", raw_output);
        }
    }

    let output = Command::new("git").arg("-v").output()?;
    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout).unwrap();
        println!("OUTPUT: {:#?}", raw_output);
    }

    Ok(())
}
