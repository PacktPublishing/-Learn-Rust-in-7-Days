extern crate rand;
use rand::{thread_rng,Rng};
use std::thread::spawn;
use std::sync::{Arc,Mutex};

#[derive(Clone,Debug)]
struct Bank {
    ///account_id = usize
    accounts:Arc<Mutex<Vec<i32>>>,
}

impl Bank {
    pub fn new(n:usize)->Self{
        let mut v = Vec::with_capacity(n);
        for _ in 0..n{
            v.push(0);
        }
        Bank{
            accounts:Arc::new(Mutex::new(v)),
        }
    }

    //Challenge: add Password Varification
    pub fn transfer(&self,from:usize,to:usize,amount:i32)->Result<(),()>{
        let mut acc = self.accounts.lock().unwrap();
        if from > acc.len() {
            return Err(());
        } 
        if to > acc.len() {
            return Err(());
        } 
        acc[from] -= amount;
        acc[to] += amount;
        Ok(())
    }
}

struct Person {
    ac_id:usize,
    buddy_id:usize,
}

impl Person{
    pub fn new(id:usize,b_id:usize)->Self{
        Person{
            ac_id:id,
            buddy_id:b_id,
        }
    }
}

fn main() {
    let bank = Bank::new(20);
    let mut handles = Vec::new();

    for i in 0..20{
        let bc = bank.clone(); 
        let p = Person::new(i,thread_rng().gen_range(0,20));
        handles.push( spawn(move ||{
            let am = thread_rng().gen_range(20,50);
            bc.transfer(p.ac_id,p.buddy_id,am).unwrap();
            
        }));
    }

    for h in handles{
        h.join().unwrap();
    }

    println!("{:?}",bank);
}
