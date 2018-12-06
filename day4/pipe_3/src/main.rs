use std::process::{Command,Stdio};
use std::io::copy;

fn main() {

    let mut c = Command::new("espeak")
                    .stdin(Stdio::piped())
                    .spawn()
                    .expect("Command didn't run");

    let mut d = Command::new("cat")
                    .arg("data/tosay.md")
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("Cat didn't run right");

    
    copy(&mut d.stdout.unwrap(),&mut c.stdin.unwrap());
    



}
