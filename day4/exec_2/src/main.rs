use std::process::Command;

fn main() {
    let mut c = Command::new("ls");
    c.arg("-l");
    let c = c.output()
                .expect("LS Not useable");

    for (n,ln) in String::from_utf8(c.stdout).expect("NOT UTF8able").split("\n").enumerate() {
        println!("Line {}: {}",n,ln);
    }



}
