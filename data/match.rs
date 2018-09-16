struct Foo { opt: Option<String>, }
impl Foo {
    fn fun(&mut self) {
        if let Some(ref val) = &self.opt {
            // immutable borrow occurs here
            println!("{}", val);
            self.gun(); // mutable borrow occurs here
        } // mutable borrow occurs here
    }
    fn gun(&mut self) { println!("BAR!"); }
}

pub fn main() {
    let mut f = Foo {
        opt: Some(String::from("Helo World!")),
    };

    f.fun();
}
