trait Animal {
    fn legs(&self) -> u32;
}


trait Pet:Animal {
    fn name(&self) -> String;
}

struct Dog(String);


impl Animal for Dog {
    fn legs(&self) -> u32 {
        4
    }
}

impl Pet for Dog {
    fn name(&self) -> String {
        self.0.clone()
    }
}


fn main() {

    let pupp = Dog(String::from("Rex"));
    println!("{} has {} legs",pupp.name(),pupp.legs());
}
