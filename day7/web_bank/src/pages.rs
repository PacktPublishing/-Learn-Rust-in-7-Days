use std::fmt::Debug;
use maud::{html,DOCTYPE,Markup};
use rocket::response::content::Html;


pub fn page(title:&str,p:Markup)->Html<String>{
    Html((html!{
        (DOCTYPE)
        html{
            head{
                meta charset = "utf-8"{ }
                title { "Mini Bank -- " (title) }
            }
            body{
                h1 {(title)}
                (p)
            }
        }
    }).into_string())
}

pub fn err_page<E:Debug>(e:E,back_to:&str)->Html<String>{
    page("Error",html!{
        p { (format!("At : {:?}",e))}
        a href = (back_to){ "Go Back" }
    })
}

pub fn home_page(uname:&str,balance:i64)->Html<String>{
    page("Home" ,html!{

        "Welcome " (uname)br; "Balance : " (balance) br; 
        
        form action="/pay" method="POST" style="border:1px solid black"{
            h2 { "Pay Somone" }
            "Payee" input name="payee" type = "text" {} br;
            "Amount" input name="amount" type = "number" {} br;
            input type="submit" value="Pay"{}
        }
    })
}

#[get("/")]
pub fn index_page()->Html<String>{
    page("Login", html!{
        form method="POST" action ="/login" style="border:1px solid black"{
            h2 { "Existing User" }
            "Username" input name="username" type = "text" {} br;
            "Password" input name="password" type = "password" {} br;
            input type="submit" value="Login"{}
        }
        form method="POST" action ="/new_user" style="border:1px solid black"{
            h2 { "New User" }
            "Username" input name="username" type = "text" {} br;
            "Password" input name="password" type = "password" {} br;
            input type="submit" value="Create"{}
        }
    })

}
