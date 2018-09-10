trait A { fn fun(&self); }
struct Foo { s: &'static str, }

impl<T> A for T {
    fn fun(&self) {
        println!("{}", self.s);
        // no field `s` on type `&T`
    }
}
