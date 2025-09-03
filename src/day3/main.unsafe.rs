fn main() {
    let mut s1 = String::from("yy ");
    s1.push_str("hhhh");

//    dbg!(s1);

    unsafe {
        let (cap,ptr,len):(usize,usize,usize) = std::mem::transmute(s1);
        println!("cap:{cap},ptr:{ptr},len:{len}");
    }
}
