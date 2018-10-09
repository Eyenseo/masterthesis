struct Foo<'a> {
  rs: &'a String,
}
fn main() {
  let foo = {
    let s = String::from("Wolle");
    Foo { rs: &s } // `s` does not
  };           // live long enough
}
