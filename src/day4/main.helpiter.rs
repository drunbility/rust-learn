fn main(){
    let result:i32 = (1..=10)
        .filter(|x|x%2==0)
        .map(|x|x*x)
        .sum();
    println!("result:{}",result);
}
