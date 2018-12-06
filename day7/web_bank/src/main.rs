#![feature(plugin,proc_macro_hygiene,custom_derive)]
#![plugin(rocket_codegen)]

use rocket::response::content::Html;
use rocket::request::Form;
use rocket::State;
use rocket::http::{Cookie,Cookies};

mod session;
use self::session::Session;
mod pages;

#[derive(Debug,FromForm)]
struct Credentials {
    username:String,
    password:String,
}

#[derive(Debug,FromForm)]
struct PayRequest {
    payee:String,
    amount:i64,
}


#[post("/new_user",data="<cred>")]
fn new_user(cred:Form<Credentials>
            ,sess:State<Session>
            ,mut cookies:Cookies
            )->Html<String>{
    let cred = cred.into_inner();
    let sess = sess.inner();
    match sess.ub.add_user(&cred.username, &cred.password) {
        Ok(_)=>{
            let n = sess.add_session(cred.username.clone());
            cookies.add(Cookie::new("session",n.to_string()));
            pages::home_page(&cred.username,0)
        },
        Err(e)=>pages::err_page(e,"/"),
    }
}

#[post("/login",data="<cred>")]
fn login(cred:Form<Credentials>
         ,sess:State<Session>
         ,mut cookies:Cookies
         )->Html<String>{
    let cred = cred.into_inner();
    let sess = sess.inner();
    
    let vd = match sess.ub.validate_password(&cred.username,&cred.password) {
        Ok(v)=>v,
        Err(e)=>return pages::err_page(e,"/"),
    };

    if !vd {
        return pages::err_page("Passwords do not match","/"); 
    }

    let sid = sess.add_session(cred.username.clone());
    cookies.add(Cookie::new("session",sid.to_string()));
    let balance = sess.ub.get_balance(&cred.username).unwrap_or(0);

    pages::home_page(&cred.username,balance)

}



#[post("/pay",data="<payreq>")]
fn pay(payreq:Form<PayRequest>
         ,sess:State<Session>
         ,cookies:Cookies
         )->Html<String>{
    let req = payreq.into_inner();
    let sess = sess.inner();

    let ck = match cookies.get("session"){
        Some(c)=>c, None=>return pages::err_page("No Cookie","/home"),
    };

    let ck:u64 = match ck.value().parse(){
        Ok(c)=>c, Err(e)=>return pages::err_page(e,"/home"),
    };

    let uname = match sess.get_session(ck) {
        Some(n)=>n, None=>return pages::err_page("No Session","/home"),
    };

    if req.amount <= 0 {
        return pages::err_page("Cannot pay negative amounts","/home");
    }

    if let Err(e) = sess.ub.pay(&uname,&req.payee,req.amount){
        return pages::err_page(e,"/home");
    }

    let ac = match sess.ub.get_balance(&uname){
        Ok(a)=>a,Err(e)=>return pages::err_page(e,"/home"),
    };
    pages::home_page(&uname,ac)
    
}

fn main() {
    let sess = Session::new("data/users.db");
    rocket::ignite()
        .manage(sess)
        .mount("/",routes![pages::index_page,new_user,login,pay])
        .launch();
}
