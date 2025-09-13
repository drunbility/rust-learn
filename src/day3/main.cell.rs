use std::cell::Cell;
use std::cell::RefCell;

fn main() {
    let cell = Cell::new(5);

    cell.set(1233);

    dbg!(cell.get());

    let c2 = RefCell::new(10);

    {
        let mut c2_r = c2.borrow_mut();
        *c2_r=123;
        //let ot  = c2.borrow();

        //dbg!(ot);
    }

    dbg!(c2);
}
