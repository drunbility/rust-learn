#[derive(Debug)]
struct Meters(i32);
#[derive(Debug)]
struct MetersSqd(i32);

trait Multi {
    type Output;
    fn multi(&self,other:&Self) -> Self::Output;
}

impl Multi for Meters {
    type Output=MetersSqd;
    fn multi(&self,other:&Self) -> Self::Output {
        MetersSqd(self.0*other.0)    
    }
}

fn main() {
    println!("{:?}",Meters(10).multi(&Meters(20)));
}
