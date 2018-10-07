struct RcBox<T: ?Sized> {
  strong: Cell<usize>,
  weak: Cell<usize>,
  value: T,
}
struct Rc<T: ?Sized> {
  ptr: NonNull<RcBox<T>>,
  phantom: PhantomData<T>,
}
struct RefCell<T: ?Sized> {
  borrow: Cell<BorrowFlag>,
  value: UnsafeCell<T>,
}
