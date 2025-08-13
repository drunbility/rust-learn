fn inter(a:i32,b:i32,c:i32)-> i32{
    return a*b;
}

fn takes_u32(x:u32){
    println!("u32:{x}");
}

fn takes_i8(y:i8){
    println!("i8:{y}");
}

fn main(){
    let mut x: i32 = 10;
    println!("x:{x}");
    println!("hello  🌍!");
    x=20;
    println!("x:{x}");
    println!("result: {}",inter(120,2,34));
    let m=20;
    let n=10;
    takes_u32(m);
    takes_i8(n);
}
