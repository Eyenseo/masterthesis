trait T { fn fun(&self) -> usize; }

#[repr(C)]
struct Foo { pub x: usize, }
#[repr(C)]
struct Bar { pub x: usize, pub y: usize, }

impl T for Foo { fn fun(&self) -> usize { self.x } }
impl T for Bar { fn fun(&self) -> usize { self.y } }

fn main() {
  let mut bar = Bar { x: 42, y: 7 };
  let foo = unsafe { std::mem::transmute::<_, *mut Foo>(&bar) };
  assert_eq!(unsafe { &*foo }.x, 42);
  assert_eq!(unsafe { &*foo }.fun(), 42);
  unsafe { &mut *foo }.x = 13;
  assert_eq!(bar.x, 13);
  assert_eq!(bar.fun(), 7);
}
