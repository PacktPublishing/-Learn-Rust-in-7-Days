//! A library for handling money


use std::str::FromStr;


pub mod parse;
use parse::*;


#[derive(Debug,PartialEq)]
pub enum GBPError {
    ParseError(ParseMoneyError),
    OtherError,
}

impl From<ParseMoneyError> for GBPError {
    fn from(p:ParseMoneyError)->Self{
        GBPError::ParseError(p)
    }
}

/// Parse your money from a string
/// ``` 
/// use doc_9_testing_12345::*;
/// let g = "£32.45".parse(); 
/// assert_eq!(g,Ok(GBP(3245)));
///
/// ```
#[derive(Debug,PartialEq)]
pub struct GBP(pub i32);

impl FromStr for GBP {
    type Err = GBPError;
    fn from_str(s:&str)->Result<Self,Self::Err>{
        Ok(GBP(parse_sym_money(s,'£',2)?))
    }
}



