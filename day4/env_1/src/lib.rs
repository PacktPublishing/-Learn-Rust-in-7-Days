use std::env::{var,set_var};

pub fn road_len()->usize{
    let e = var("ROAD").unwrap_or("".to_string());
    e.len()
}

pub fn rail_len()->usize{
    let s = var("GWR").unwrap_or("".to_string());
    _rail_len(&s)
}

fn _rail_len(s:&str)->usize{
    s.len() 
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        //let e = var("HELLO").unwrap();
        //assert_eq!(&e,"WORLD");
        
        set_var("ROAD", "Route66");
        let e = road_len();
        assert_eq!(e,7);

        let r = _rail_len("Pointless Track");
        assert_eq!(r,15);
    }
}
