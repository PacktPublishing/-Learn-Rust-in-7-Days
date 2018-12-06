extern crate bcrypt;

use bcrypt::{hash,verify,BcryptError};


#[derive(Debug)]
pub struct User{
    uname:String,
    pass_hash:String,
}

impl User{
    pub fn new(uname:String,pwd:&str)->Result<User,BcryptError>{
        Ok(User{
            uname,
            pass_hash:hash(pwd,6)?,
        })
    }

    pub fn verify(&self, pwd:&str)->bool {
        verify(pwd,&self.pass_hash).unwrap_or(false)
    }
}

fn main() {
    let u = User::new("pete".to_string(),"pete_pw").unwrap();
    println!("{:?}",u);

    let p1 = "pete_pw";
    println!("{}:{}",p1,u.verify(p1));
    
    let p2 = "pete_p2";
    println!("{}:{}",p2,u.verify(p2));
}
