fn main() {
    let mut vec = vec![1,2,3,4,5];
    let elm = &vec[2];
    vec.push(6);
    dbg!(elm);
    //for el in &vec {
    //    vec.push(el+2);
    //}
}
