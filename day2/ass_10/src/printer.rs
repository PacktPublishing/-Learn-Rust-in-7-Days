


pub fn display_curr(mut v:i32,sym:char,dp:usize)->String{
    let mut res = "".to_string();
    let minus =  v < 0;
    if minus {
        v*= -1;
        res.push_str("-");
    }
    res.push(sym);

    let s = v.to_string();
    if s.len() < dp {
        // not enough digits to reach dp
        res.push_str("0.");
        for _ in s.len()..dp{
            res.push('0');
        }
        res.push_str(&s);
        return res;
    }
    
    //enough digits: split and join
    let (a,b) = (&s).split_at(s.len()-dp);
    res.push_str(a);
    res.push('.');
    res.push_str(b);
    res
    
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    pub fn print_em(){
        assert_eq!(&display_curr(-3,'£',2),"-£0.03");
        assert_eq!(&display_curr(-3456,'£',2),"-£34.56");

        assert_eq!(&display_curr(3,'£',2),"£0.03");
        assert_eq!(&display_curr(3456,'£',2),"£34.56");
        
    }
}
