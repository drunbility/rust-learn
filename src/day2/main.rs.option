fn main() {
    let name = "fdsfdasfasdff";
    let mut posi:Option<usize>  = name.find('a');
    dbg!(posi);
    assert_eq!(posi.unwrap(),5);
    posi=name.find('2');
    //dbg!(posi);
    assert_eq!(posi.expect("not"),0);
}
