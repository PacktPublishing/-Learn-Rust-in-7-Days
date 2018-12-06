use std::thread::sleep;
use std::time::Duration;
use rayon::prelude::*;
use rayon::join;


pub fn square_split(v:&mut [i32]){
    if v.len() < 4 {
        for i in v {
            sleep(Duration::from_millis(100));
            println!("{}",*i);
            *i *= *i;
        }
        return;
    }
    let (mut a,mut b) = v.split_at_mut(v.len()/2);
    join(|| square_split(&mut a), || square_split(&mut b));
}


fn main() {
    
    let mut v = Vec::with_capacity(500);
    for i in 0..500 {
        v.push(i);
    }

    square_split(&mut v);
    /*v.par_iter_mut().for_each(|x|{
        sleep(Duration::from_millis(100));
        println!("{}",*x);
        *x *= *x
    });*/

    println!("{:?}",&v[490..500]);
}
