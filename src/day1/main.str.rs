fn main() {
    let s1:&str = "world";
    println!("s1:{s1:?}");
    let mut s2:String = String::from("test ");
    println!("s2:{s2}");
    s2.push_str(s1);
    println!("s2:{s2}");
    let s3:&str = &s2[2..10];
    println!("s3:{s3}");
    println!("{:?}",b"abc");    
    println!("{:?}",&[97,98,99]);
    println!(r#"<a href="link.html">link</a>"#);
    println!("<a href=\"link.html\">link</a>");
    
}
