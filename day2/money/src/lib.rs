
#[derive(PartialEq,Debug)]
pub struct USD (i32);
#[derive(PartialEq,Debug)]
pub struct GBP (i32);
#[derive(PartialEq,Debug)]
pub struct CAD (i32);

pub trait ToUSD {
    fn to_usd(&self)->USD ;
    fn convert<T:FromUSD>(&self)->T{
        T::from_usd(&self.to_usd())
    }
}

impl ToUSD for GBP {
    fn to_usd(&self)->USD{
        USD((self.0 * 130) / 100)
    }
}


pub trait FromUSD {
    fn from_usd(&USD)->Self;
}

impl FromUSD for CAD{
    fn from_usd(u:&USD)->Self {
        CAD((u.0 * 130) / 100)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let g = GBP(200);
        let u = g.to_usd();
        assert_eq!(u, USD(260));
        let c = CAD::from_usd(&u);
        assert_eq!(c, CAD(338));

        let c2:CAD = g.convert();
        assert_eq!(c2,c);
    }
}
