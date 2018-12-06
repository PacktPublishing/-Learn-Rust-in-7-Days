
fn main() {
    println!("Hello, world!");
    //loop_to_10();
    array_loop();
}

fn loop_to_10(){
    for n in 0..10 {
        println!("Hello {}",n);
    }
}

fn array_loop(){

    let v = vec![4,7,8,9,11,14];

    'outer: for i in 1..10 {
        for n in &v {
            if i+n ==11  {
                break 'outer;
            }
            println!("{}",n);
        }
    }
}


