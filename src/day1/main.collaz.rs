fn collatz_length(mut n: i32) -> u32 {
  
  let mut cnt = 1;
  while n != 1 {
      n = match n%2 {
          0 => n/2,
          _ => 3*n+1,
      };
      cnt+=1;
  };
  cnt
}

fn main() {
    println!("Length: {}", collatz_length(11)); // should be 15
}
