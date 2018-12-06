
#[derive(Debug)]
pub struct Bed{  
    size:i32,
    count:u32,
}

#[derive(Debug)]
pub enum Room{
    Kitchen(i32),
    Bedroom(Bed),
    Lounge(i32,String),
}

fn main() {
    use self::Room::*;
    let t = Lounge(5,"big".to_string());
    println!("Hello from the {:?}",t);

    if let Lounge(n,s) = t {
        println!("Its a {} lounge with {} cupboards",s,n);
    }
}
