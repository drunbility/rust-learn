fn main() {
    let mut point = (1,2);
    let x = &mut point.0;
    *x = 11;
    x =  &point.1;
    println!("point:{point:?}");
}
