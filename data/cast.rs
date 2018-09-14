fn cast<U>(&self) -> Option<Ptr<U>>
where
    U: ?Sized + 'static + CastAble,
{
    let own_id = unsafe { self.meta.as_ref() }.cast_id;
    if let Some(vtable) = VALID_CASTS[own_id][U::cast_id()] {
        unsafe {
            let mut t = TraitObject {
                data: self.value.as_ptr() as *const (),
                vtable,
            };
            let value = NonNull::new_unchecked(
                *::std::mem::transmute::<_, &mut *mut U>(&mut t),
            );
            let meta = self.meta.as_ref();
            meta.strong.set(meta.strong.get() + 1);
            Some(Ptr { meta: self.meta, value })
        }
    } else {
        None
    }
}

