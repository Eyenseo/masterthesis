struct MetaData {
    cast_id: usize,
    strong: Cell<usize>,
    weak: Cell<usize>,
    borrow: Cell<BorrowFlag>,
}
struct Ptr<T>
where
    T: ?Sized,
{
    meta: NonNull<MetaData>,
    value: NonNull<T>,
}
