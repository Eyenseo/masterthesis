struct Counter {
  count: usize,
}
impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}
impl Iterator for Counter {
  type Item = usize;

  fn next(&mut self) -> Option<usize> {
    self.count += 1;
    if self.count < 6 {
      Some(self.count)
    } else {
      None
    }
  }
}
struct Foo {
  counter: Counter,
}
impl Foo {
  fn fun(&mut self) {
    for i in &mut self.counter { // cannot move out of borrowed content
      println!("{}", i);
    }
    loop {
      if let Some(i) = self.counter.next() {
        println!("{}", i);
      } else {
        break;
      }
    }
  }
}
fn main() {
  let mut x = Foo {
    counter: Counter::new(),
  };

  x.fun();
}
