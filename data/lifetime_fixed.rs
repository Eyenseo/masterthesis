struct Foo<'a> {
  s: &'a String,
}
pub fn main() {
  let foo = {
    let s = String::from("Wolle");
    Foo { s: &s } // `s` does not
                  // live long
                  // enough
  };
}
