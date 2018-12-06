use std::env::args;
use std::io::stdin;
use minibankdb::{UserBase,UBaseErr};

#[derive(Debug)]
enum AppErr {
    Mess(String),
    UBaseErr(UBaseErr),
}

impl From<UBaseErr> for AppErr{
    fn from(u:UBaseErr)->Self{
        AppErr::UBaseErr(u)
    }
}

impl From<&str> for AppErr{
    fn from(s:&str)->Self{
        AppErr::Mess(s.to_string())
    }
}

impl From<String> for AppErr{
    fn from(s:String)->Self{
        AppErr::Mess(s)
    }
}

fn input_password()->Result<String,AppErr>{
    println!("Please input your password");
    let mut res = String::new();
    stdin().read_line(&mut res).map_err(|_|"Could not read password")?;
    Ok(res.trim().to_string())
}

fn _main()->Result<(),AppErr>{
    let mut args = args();
    args.next(); //drop progname

    let ub = UserBase::new("test_data/users.db");

    let job = args.next().ok_or("please include job -[create,pay,view]")?;
    match job.as_ref(){
        "create"=>{
            let uname = args.next().ok_or("need name")?;
            let pw = args.next().ok_or("need password")?;
            println!("Adding:{}:{}:",uname,pw);
            ub.add_user(uname.trim(),pw.trim())?;
             
        },
        "pay"=>{
            let from = args.next().ok_or("need from")?;
            let to_ac = args.next().ok_or("need to_account")?;
            let am_str = args.next().ok_or("need amount")?;
            
            let pw = input_password()?;
            if ! ub.validate_password(from.trim(),pw.trim())?{
                return Err("Password incorrect".into());
            }
            ub.pay(&from,&to_ac,am_str.parse().map_err(|_|format!("Not a number:{}",am_str))?)?;
        }
        "view"=>{
            let uname = args.next().ok_or("need user name")?;
            let pw = input_password()?;
            if ! ub.validate_password(uname.trim(),pw.trim())?{
                return Err("Password incorrect".into());
            }
            println!("Balance = {}",ub.get_balance(uname.trim())?);
        },
        e=>{println!("Did not understand - {:?}",e)},
    }
    Ok(())
}

fn main() {
    match _main() {
        Ok(_)=>println!("Done"),
        Err(e)=>println!("Fail : {:?}",e),
    }

}
