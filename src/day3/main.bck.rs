fn main() {
    //let x_ref = {
    //    let x =10;
    //    &x
    //};

    //dbg!(x_ref);
    let mut a = 10;

    let b = &a;
        
    {
        let c = &mut a;
        *c = 5;
    }
    dbg!(a);
    //dbg!(b);
}
