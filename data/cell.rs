use std::cell::Cell;

struct Foo { x: Cell<i8>, y: i8 }

fn main() {
    let f = Foo { x: Cell::new(42), y: 7 };
    f.x.set(13);
    f.y = 13; // cannot mutably borrow field of immutable binding
}
