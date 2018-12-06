
#[derive(Debug,PartialEq)]
pub struct GBP(i32);


/*fn money_pointer<'a>(i:i32)->&'a GBP{
    let g = GBP(i);
    &g
}*/

pub fn on_money(a:i32,b:i32)->GBP {
        let mut g = GBP(a);
    let r;
    
        r = &g;
    

    let res = GBP(r.0 + b);
    res
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
      //  let g = money_pointer(3);
      //  assert_eq!(*g, GBP(3));
        let g = on_money(3,4);
        assert_eq!(g, GBP(7));
    }
}
