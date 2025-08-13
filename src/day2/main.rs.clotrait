
fn apply_and_log (
    func:impl FnOnce(&'static str) -> String,
    func_name: &'static str,
    input: &'static str,
) {
    println!("calling {func_name}({input}): {}",func(input))
}

fn main() {
    let suffix  = "-it";
    let add_suffix = |x| format!("{x}{suffix}");

    apply_and_log(&add_suffix,"add_suffix","senior");

    apply_and_log(&add_suffix,"add_suffix","appendix");

    let mut v = Vec::new();
    let mut acc  = |x| {
        v.push(x);
        v.join("/")
    };
    apply_and_log(&mut acc,"acc","red");
    apply_and_log(&mut acc,"acc","green");
    apply_and_log(&mut acc,"acc","blue");

    let take_and_reverse = |prefix| {
        let mut acc = String::from(prefix);
        acc.push_str(&v.into_iter().rev().collect::<Vec<_>>().join("/"));
        acc
    };


    apply_and_log(take_and_reverse,"take_and_rever","reversed: ");
    
}
