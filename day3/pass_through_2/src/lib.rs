
pub fn trim_left<'a>(s:&'a str)->&'a str {
    for (i ,c ) in s.char_indices() {
        if c == ' ' {
            continue;
        }
        return s.get(i..s.len()).unwrap();
    }
    ""
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(trim_left("    hello"),"hello");

        let mut s = "  hello".to_string();
        {
            let s2 = trim_left(&s);
            assert_eq!(s2, "hello");
        }

        s.push_str(" world");
        assert_eq!(trim_left(&s), "hello world");
    }
}
