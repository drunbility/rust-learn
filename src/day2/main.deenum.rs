enum Result {
    Ok(i32),
    Err(String),
    Unk {msg:String,code:i32},
}

fn divide_in_two(n:i32) -> Result {
    if n%3==0 {
        Result::Ok(n/3)
    }else if n%3==1 {
        Result::Unk{msg:String::from("ttt"),code:444}
    } else {
        Result::Err(format!("cannot divide {n}"))
    }
}



fn main() {
    let n = 100;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("err:{msg}"),
        Result::Unk{msg,code} => println!("code:{code}"),
    }    
}
