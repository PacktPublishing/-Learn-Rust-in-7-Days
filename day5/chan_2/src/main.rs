use std::thread::spawn;
use std::sync::mpsc;



fn main() {
    let (cs,cr) = mpsc::channel::<i32>();
    let h = spawn(move||{
        loop {
            match cr.recv() {
                Ok(v) => {
                    println!("Value {}",v);
                },
                Err(e) =>{
                    println!("Err = {:?}",e);
                    return;
                }
            }
        }
    });

    let cs2 = cs.clone();
    
    spawn(move|| {
        for j in 10 ..20 {
            cs2.send(j).unwrap();
        }
    });

    for i in 0..10{
        cs.send(i).expect("Reciever dropped early");
    }

    drop(cs);

    h.join();




}
