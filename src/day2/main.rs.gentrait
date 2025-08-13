
#[derive(Debug)]
struct Foo(String);

impl From<u32> for Foo {
    fn from(from:u32) -> Foo {
        Foo(format!("from inter: {from}"))
    } 
}


impl From<bool> for Foo {
    fn from(from:bool) -> Foo {
        Foo(format!("from bool: {from}"))
    }
}

fn main() {
    let from_int = Foo::from(123);
    let from_bool = Foo::from(true);
    dbg!(from_int);
    dbg!(from_bool);
}

//#[derive(Debug)]
// struct Foo(String);

// impl From<u32> for Foo {
//     fn from(from: u32) -> Foo {
//         Foo(format!("Converted from integer: {from}"))
//     }
// }

// impl From<bool> for Foo {
//     fn from(from: bool) -> Foo {
//         Foo(format!("Converted from bool: {from}"))
//     }
// }

// fn main() {
//     let from_int = Foo::from(123);
//     let from_bool = Foo::from(true);
//     dbg!(from_int);
//     dbg!(from_bool);
// }
