
#[derive(Debug)]
struct User{
    name:String,
    age:i32,
    height:i32,
    shoesize:i32,
}

impl User{
    fn simple_string(&self)->String{
        format!("{} - {} - {}cm - shoe:{}",self.name,self.age,self.height,self.shoesize) 
    }

    fn grow(&mut self,h:i32){
        self.height += h;
    }

    fn die(self){
        println!("Dead {} ", self.simple_string());
    }
}


fn main() {

    let mut u = User{
        name:"Matt".to_string(),
        age:33,
        height:250,
        shoesize:10,
    };

    println!("User is {}",u.simple_string());
    u.grow(20);
    println!("User is {}",u.simple_string());


}
