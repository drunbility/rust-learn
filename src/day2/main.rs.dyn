struct Dog {
    name:String,
    age:i8,
}

struct Cat {
    lives:i8,
}

trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("woo,{}",self.name)
    }
    
}

impl Pet for Cat {
    fn talk(&self) -> String {
        String::from("Minu")
    }
}

fn generic(pet:&impl Pet) {
    println!("hello:{}",pet.talk());
}

fn dyna(pet:&dyn Pet) {
    println!("you : {}",pet.talk());
}

fn main() {
    let cat = Cat { lives:9};
    let dog = Dog {name:String::from("fido"),age:5};

    generic(&cat);
    generic(&dog);

    dyna(&cat);
    dyna(&dog);
}
