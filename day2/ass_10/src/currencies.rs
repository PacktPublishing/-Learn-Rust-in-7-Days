use parse::*;
use std::str::FromStr;
use std::fmt;
use printer::display_curr;
use std::ops::AddAssign;
use std::ops::Neg;

#[derive(Copy,Clone)]
pub struct Money<T:Currency>{
    c:T,
}


impl<T:Currency> fmt::Debug for Money<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",display_curr(self.c.value(),T::symbol(),T::decimal_point()))
    }
}

impl<T:Currency> fmt::Display for Money<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",display_curr(self.c.value(),T::symbol(),T::decimal_point()))
    }
}

impl<T:Currency> PartialEq for Money<T>{
    fn eq(&self,other:&Self)->bool{
        self.value() == other.value()
    }
}

pub trait Currency{
    fn symbol()->char { '$' }
    fn decimal_point()->usize{ 2 } 
    fn from_value(v:i32)->Self;
    fn value(&self)->i32;
}

impl<T:Currency> Currency for Money<T>{
    fn symbol()->char { T::symbol() }
    fn decimal_point()->usize{ T::decimal_point() } 
    fn from_value(v:i32)->Self{ Money{c:T::from_value(v)}}
    fn value(&self)->i32{ self.c.value()}
}

impl<T:Currency> From<T> for Money<T>{
    fn from(t:T)->Self{
        Money{c:t}
    }
}

impl<T:Currency> FromStr for Money<T>{
    type Err =  ParseMoneyError;
    fn from_str(s:&str)->Result<Self,Self::Err>{
        let v = parse_sym_money(s,T::symbol(),T::decimal_point())?;
        Ok(Self::from_value(v))
    }
}

impl<T:Currency> AddAssign for Money<T>{
    fn add_assign(&mut self, other:Self){
        self.c = T::from_value(self.value()+other.value());
    }
}


impl<T:Currency> Neg for Money<T>{
    type Output = Self;
    fn neg(self)->Money<T>{
        Money::from_value(-self.value())
    }
}
/// Parse your money from a string
/// ``` 
/// use money_typesafe::*;
/// use money_typesafe::currencies::*;
/// let g:Money<GBP> = "£32.45".parse().unwrap(); 
/// assert_eq!(g,GBP(3245).into());
///
/// ```
#[derive(Copy,Clone)]
pub struct GBP(pub i32);


impl Currency for GBP{
    fn symbol()->char{'£'}
    fn from_value(v:i32)->Self{
        GBP(v)
    }
    fn value(&self)->i32{
        self.0
    }
}


#[derive(Copy,Clone)]
pub struct CAD(pub i32);
impl Currency for CAD{
    fn from_value(v:i32)->Self{
        CAD(v)
    }
    fn value(&self)->i32{
        self.0
    }
}

#[derive(Copy,Clone)]
pub struct USD(pub i32);
impl Currency for USD{
    fn from_value(v:i32)->Self{
        USD(v)
    }
    fn value(&self)->i32{
        self.0
    }
}


