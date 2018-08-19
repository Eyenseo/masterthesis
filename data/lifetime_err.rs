struct Foo {
  s: &String, // ERROR missing
              // lifetime
              // specifier
}
pub fn main() {
  let foo = {
    let s = String::from("Hello");
    Foo { s: &s }
  };
}
