
#[derive(Clone,Debug)]
struct Point(i32,i32,String);

fn say(name:String){
    println!("hi , {name}");
}


fn main() {
    let s1 = String::from("aaaa");

    let s2  = s1;

    dbg!(s2);
    //dbg!(s1);
    //

    let n = String::from("fffff");
    say(n.clone());
    say(n);
    let x = 42;
    let y = x;
    dbg!(x);
    dbg!(y);

    let p1 = Point(32,21,String::from("ggg"));
    let p2 =p1.clone();
    println!("p1:{p1:?}");
    println!("p2:{p2:?}");
    
}
