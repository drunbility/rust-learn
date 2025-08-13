#[rustfmt::skip]
fn main() {
    let input = 'x';
    match input {
        'q'    => println!("exit"),
        'a'|'s'|'w'|'d'   => println!("moving"),
        '0'..='9'     => println!("number in"),
        key if key.is_lowercase()   => println!("lower case: {key}"),
        _                           => println!("else"), 
    }
}
