trait Pet {
    fn talk(&self) -> String;
    fn greet(&self) {
        println!("greet:{}",self.talk());
    }
}

struct Dog {
    name:String,
    age:i8,
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("woof , {}",self.name)
    }
}

fn main() {
    let fido = Dog { name:String::from("fido"),age:5 };

    dbg!(fido.talk());
    fido.greet();
}
