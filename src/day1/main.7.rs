fn check_order(tuple:(i32,i32,i32)) -> bool{
    let (r,m,l) = tuple;
    r < m && m < l 
}

fn main() {
    let t = (1,3,5);

    println!("{t:?}:{}",if check_order(t) {"or"} else {"not"});

}
