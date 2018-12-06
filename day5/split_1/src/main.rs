use std::thread::{spawn,sleep};
use std::time::Duration;

fn main() {
    let h = spawn(|| {
        for i in 0..10 {
            sleep(Duration::from_millis(10));
            println!("{}",i);
        }
        return 5;
    });

    for i in 10..20 {
        sleep(Duration::from_millis(20));
        println!("{}",i);
    }

    let r = h.join().unwrap();
    println!("Done R = {}",r);
}
