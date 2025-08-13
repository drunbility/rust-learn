#[derive(Clone,Debug,Copy)]
struct Point {
    x:i32,
    y:i32,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self,other:Self) -> Point{
        Self{x:self.x + other.x,y:self.y+other.y}
    }
}

fn main() {
    let p1 = Point{x:12,y:13};
    let p2 = Point{x:3,y:4};

    println!("{p1:?}+{p2:?} = {:?}",p1+p2);
}
