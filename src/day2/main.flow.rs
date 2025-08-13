use std::time::Duration;

fn sleep_for(secs:f32) {
    let result = Duration::try_from_secs_f32(secs);

    if let Ok(duration) = result {
        std::thread::sleep(duration);
        println!("slept for {duration:?}");
    };
}

fn hex_or_die_trying(maybe_string:Option<String>) -> Result<u32,String> {
    let Some(s) = maybe_string else {
        return Err(String::from("got none"));
    };

    let Some(first) = s.chars().next()  else {
        return Err(String::from("got empty"));  
    };

    let Some(digit) = first.to_digit(16) else {
        return Err(String::from("not a hex digit"));
    };
    Ok(digit)
}

fn main() {
    sleep_for(-10.0);
    sleep_for(0.9);
    //let mut name = String::from("comprehensive");
    //while let Some(c) = name.pop() {
    //    dbg!(c);
    //}
    println!("result: {:?}",hex_or_die_trying(Some(String::from("foo"))));
}
