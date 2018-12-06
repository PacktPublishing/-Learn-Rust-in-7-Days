use std::sync::{Mutex,Arc};
use std::thread::spawn;



fn main() {
    println!("Hello, world!");

    let m = Arc::new(Mutex::new(32));

    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {
        let am= m.clone();
        let j = i;
        handles.push(spawn(move ||{
            let mut p = am.lock().unwrap();
            *p += j;
            println!("j = {} , p = {}" ,j,*p);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Done");
}
