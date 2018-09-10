use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    pub s: &'static str,
    pub next: Vec<Weak<RefCell<Node>>>,
}
pub fn main() {
    let a = Rc::new(RefCell::new(Node { s: "Lore", next: Vec::default() }));
    let b = Rc::new(RefCell::new(Node { s: "Steffi", next: Vec::default() }));
    b.borrow_mut().next.push(Rc::downgrade(&a));
    a.borrow_mut().next.push(Rc::downgrade(&b));
    b.borrow_mut().s = "Benni";
}
