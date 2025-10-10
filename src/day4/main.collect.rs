fn main() {
    let pris = vec![2,3,5,7];
    let sqs:Vec<_> = pris.into_iter().map(|x|x*x).collect();
    dbg!(sqs);
}
