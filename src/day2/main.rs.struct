struct Foo {
    x:(u32,u32),
    y:u32,
}


#[rustfmt::skip]
fn main() {
    let foo = Foo {y:3,x:(3,1)};
    match &foo {
        Foo {y:2,x:i} => println!("y=2,x={i:?}"),
        Foo {x:(1,b),y} => println!("x.0=1,b={b},y={y}"),
        Foo {y,..} => println!("y={y},other ignored"), 
    }
}
