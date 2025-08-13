fn cal_lenth(s:&String) -> usize {
    s.len()
}

fn change(so:&mut String) {

    so.push_str(",ttt");
}


fn main() {
    let s1 = String::from("test");
    let s2 = s1;
    println!("s2:{s2}");

    let len = cal_lenth(&s2);

    println!("len:{len}");

    let mut s3 = String::from("test");

    //change(&mut s3);
    let b = &mut s3;
    let b2 = &mut s3;
    println!("s3:{s3}");
}
