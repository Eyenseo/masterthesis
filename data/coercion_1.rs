struct Foo;
trait Bar {}
impl Bar for Foo {}

fn fun(f: &Foo) {}
fn gun<T: Bar>(f: T) {}
fn nun<T: Bar>(f: Box<T>) {}
fn pun(f: Box<Bar>) {}

fn main() {
  let mut f = Foo;
  fun(&mut f); // coersion
  fun(&f); // coersion
  fun(f); // expected &Foo,
      // found struct `Foo`
  gun(f); // coersion

  let mut f = Box::new(Foo);
  nun(f); // coersion
  pun(f); // coersion
}
