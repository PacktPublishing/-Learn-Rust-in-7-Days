#![feature(plugin,proc_macro_hygiene)]
#![plugin(rocket_codegen)]

use rocket::response::content::Html;
use rocket::response::NamedFile;
use std::path::PathBuf;
use std::io;

#[get("/")]
fn index()->Html<String>{
    Html("<H1>Hello</h1><p>world</p>".to_string())
}

#[get("/s/<path..>")]
fn static_file(path:PathBuf)->io::Result<NamedFile>{
    let mut pb = PathBuf::from("site/");
    pb.push(&path);
    NamedFile::open(pb)
}

fn main() {
    rocket::ignite()
        .mount("/",routes![index,static_file])
        .launch();
}
