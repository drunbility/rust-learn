fn add_42(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}


fn pair(x:u32) -> impl std::fmt::Debug {
    (x+1,x-1)
}


fn main() {
    let many= add_42(42_i8);
    dbg!(many);
    let mor = add_42(10_000_000);
    dbg!(mor);
    let de = pair(27);
    //let de:()=..;
    dbg!(de);
}
