fn pick<T>(cond:bool,left:T,right:T) -> T {
    if cond {left} else {right}
}

fn dup<T:Clone>(a:T) -> (T,T) {
    (a.clone(),a.clone())
}

struct NotCloneable;

fn main() {
    println!("picked a number:{:?}",pick(true,222,333));
    println!("picked a string:{:?}",pick(false,'L','R'));

    let foo = String::from("foo");
    let p = dup(foo);
    println!("p:{p:?}");
    //println!("foo:{foo:?}");
    let n = NotCloneable;
    //dup(n);
    
}
