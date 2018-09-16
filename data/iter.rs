struct Counter { count: usize, }
impl Counter { fn new() -> Counter { Counter { count: 0 } } }
impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.count += 1;
        if self.count < 6 { Some(self.count) }
        else { None }
    }
}

struct Foo { counter: Counter, }
impl Foo {
    fn fun(&mut self) {
        // cannot move out of borrowed content
        for i in self.counter { println!("{}", i); }
        loop {
            if let Some(i) = self.counter.next() {
                println!("{}", i);
            }
            else { break; }
        }
    }
}

pub fn main() {
    let mut f = Foo { counter: Counter::new(), };

    f.fun();
}
