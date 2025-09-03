struct Point(i32,i32);

fn main() {
    {
        let p = Point(4,2);
        dbg!(p.0);
    }
    dbg!(p.1);
}
