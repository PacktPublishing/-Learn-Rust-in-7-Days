use std::fmt::Display;
use std::fmt;


#[derive(Debug,PartialEq,Serialize,Deserialize)]
pub struct USD(i32);


#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub struct Transaction<A>{
    from_id:i32,
    to_id:i32,
    amount:A,
}



impl Display for USD {
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        let r = (self.0 as f32) / 100.;
        if r < 0. {
            return write!(f,"-${:.2}",-r);
        }
        write!(f,"${:.2}",r)
    }
}

impl Clone for USD{
    fn clone(&self)->USD{
        USD(self.0 + 1)
    }
}

impl Copy for USD{
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let u = USD(230);
        let b = u;

        assert_eq!(u,b);

        assert_eq!(u.to_string(), "$2.30".to_string());


        let t = Transaction{
            from_id :10,
            to_id:20,
            amount:USD(300),
        }

        assert_eq!(
        //TODO MAKE JSON work, import and call function

    }
}
