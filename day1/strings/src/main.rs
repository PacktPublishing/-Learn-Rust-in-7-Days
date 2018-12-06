fn main() {

    let s = String::from("Hello 中国");

    println!("S byte len = {}",s.len());

    println!("Num L's = {}",count_l(&s));

    for c in s.chars(){
        println!("{}",c);
    }

    for c in s.bytes(){
        println!("{}",c);
    }

    for (i,c) in s.char_indices(){
        println!("{} = {}",i,c);
    }
}

fn count_l(s:&str)->i32{
    let mut res = 0;
    for c in s.chars(){
        if c == 'l' {
            res +=1;
        }
    }
    res
}
