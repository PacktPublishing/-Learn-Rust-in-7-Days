use std::fs::File;
use std::io::{Read,Write,copy};


fn main()->std::io::Result<()> {
    let mut s = String::new();

    let mut f = File::open("data/from.md")?;
    f.read_to_string(&mut s)?;

    println!("{}",s);


    let mut t = File::create("data/to.md")?;

    //copy(&mut f,&mut t)?;
    
    t.write_all(&s.into_bytes())?;

    Ok(())
}
