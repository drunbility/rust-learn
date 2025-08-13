struct Person {
    name: String,
    age: u8,
}

struct Point(i32,i32);

struct Pound(f64);
struct Nett(f64);

fn describe(per: &Person){

    println!("{} is {} years old.",per.name,per.age);
}

fn main(){

    let mut per = Person {
        name: String::from("peter"),
        age: 20,
    };

    describe(&per);
    per.name=String::from("jack");
    describe(&per);
    let name= String::from("ma");
    let age = 10;
    let p1 = Person {name,age };
    describe(&p1);
    let p = Point(2,3);
    println!("p:({},{})",p.0,p.1);
}
