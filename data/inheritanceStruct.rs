trait T {
    fn fun(&self) -> usize;
}

#[repr(C)]
struct Foo {
    pub a: usize,
}
#[repr(C)]
struct Bar {
    pub a: usize,
    pub b: usize,
}

impl T for Foo {
    fn fun(&self) -> usize {
        self.a
    }
}
impl T for Bar {
    fn fun(&self) -> usize {
        self.b
    }
}

fn main() {
    let mut bar = Bar { a: 42, b: 7 };
    let f = unsafe { std::mem::transmute::<_, *mut Foo>(&bar) };
    assert_eq!(unsafe { &*f }.a, 42);
    assert_eq!(unsafe { &*f }.fun(), 42);
    unsafe { &mut *f }.a = 13;
    assert_eq!(bar.a, 13);
    assert_eq!(bar.fun(), 7);
}
