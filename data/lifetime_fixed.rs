struct Foo<'a> {
  s: &'a String,
}
pub fn main() {
  let foo = {
    let s = String::from("Hello");
    Foo { s: &s } // ERROR `s` does
                  // not live long
                  // enough
  };
}
