#[derive(Debug)]
enum HighColor {
    Pink,
    Yellow,
}


#[derive(Debug)]
struct Highlight<'doc> {
    slice:&'doc str,
    color:HighColor,
}


fn main() {
    let doc = String::from("quick lazy dog");
    let noun = Highlight{slice:&doc[6..9],color:HighColor::Pink};
    let verb = Highlight {slice:&doc[2..5],color:HighColor::Yellow};
    //drop(doc);
    dbg!(noun);
    dbg!(verb);
    
}
