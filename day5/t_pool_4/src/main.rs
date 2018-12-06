use std::thread::{JoinHandle,spawn,sleep};
use std::sync::mpsc::{channel,Sender,Receiver};
use std::sync::{Arc,Mutex};
use std::time::{Duration};


trait FnBox {
    fn call_box(self:Box<Self>);
}

impl<F:FnOnce()> FnBox for F{
    fn call_box(self:Box<Self>){
        self();
    }
}



type Job = Box<FnBox + 'static + Send>;


pub struct ThreadPool{
    handles:Vec<JoinHandle<()>>,
    ch:Sender<Job>,
}

impl ThreadPool{
    pub fn new(n:usize)->Self{
        let (cs,cr ) = channel::<Job>();
        let amap = Arc::new(Mutex::new(cr));
        let mut handles = Vec::with_capacity(n);
        for _ in 0..n{
            let amp = amap.clone();
            handles.push(spawn(move ||{
                loop {
                    let job = match amp.lock().unwrap().recv(){
                        Ok(j)=>j,
                        _=>return,
                    };
                    job.call_box();
                }
            }));
        }

        ThreadPool {
            handles:handles,
            ch:cs,
        }
    }

    pub fn add<F:FnOnce() +'static + Send>(&self,f:F){
        self.ch.send(Box::new(f)).unwrap();
    }

    pub fn end(self){
        drop(self.ch);
        for h in self.handles {
            h.join();
        }
    }
}



fn main() {
    let p = ThreadPool::new(10);

    for i in 0 ..100 {
        let j = i;
        p.add(move||{
            sleep(Duration::from_millis(500));
            println!("J = {}",j);
        })
    }

    p.end();
    println!("DONE");


}
