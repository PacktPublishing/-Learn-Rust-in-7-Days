#![feature(proc_macro_hygiene)]

use maud::{Markup, html,DOCTYPE};

fn page(title:&str,m:Markup)->String{
    (html!{
        (DOCTYPE) 
        html {
            head {
                title { (title)}
                meta charset = "utf-8" {}
            }
            body{
                (m)
            }
        }
    }).into_string()
}

fn home(uname:&str, amount:i64)->Markup{
    html!{
        h1{ "Welcome : " (uname) }
        p { "You have : £" (amount)}
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_page(){
        assert_eq!(&page("mainpage",home("dave",34)),"<!DOCTYPE html><html><head><title>mainpage</title><meta charset=\"utf-8\"></meta></head><body><h1>Welcome : dave</h1><p>You have : £34</p></body></html>");
    }
}
