fn cast<U>(&self) -> Option<Ptr<U>>
where
  // Bound of U;
  //   U can be Sized, or Unsized
  //   U may only have references that life as long as the whole program
  //   U has to implement CastAble which is needed to call cast_id()
  U: ?Sized + 'static + CastAble,
{
  // Obtain the lookup table index of Self
  let own_id = unsafe { // Use of unsafe as a pointer is accessed
    self.meta.as_ref() // Obtain a reference to the MetaData struct
  }.cast_id; // Access the value through the reference
  // Access the lookup table with the index of Self and U
  let res = VALID_CASTS[own_id][U::cast_id()];
  // If the cast is valid unwrap res
  if let Some(vtable) = res { // vtable is the vtable of U's trait
    unsafe { // Unsafe because of pointers and transmute
      let mut t = TraitObject { // Construct a new TraitObject
        // value is either a pointer to a Sized type e.g. Struct or of a
        //   trait object. In both cases points the pointer to data, a
        //   Sized type
        data: self.value.as_ptr() as *const (),
        vtable, // Set the vtable of U
      };
      // Create a new wrapped pointer
      let value = NonNull::new_unchecked(
        // Reinterpret the memory pointed to by ts reference;
        //   If U is Unsized, a Trait, then the type of the reference will
        //     be reinterpreted to a real trait object of type U
        //   If U is a Sized type, e.g. Struct the reference will be
        //     reinterpreted as reference to U
        //     The vtable pointer will be 'cut off' in this case
        *::std::mem::transmute::<
          _, // Type of the argument (&mut *mut T)
          &mut *mut U // A reference to a mutable pointer that points to a
                      //   mutable U
        >(&mut t), // Take a reference to mutable t
      );
      let meta = self.meta.as_ref(); // Convenience variable
      // Increase the strong counter for the reference counting
      meta.strong.set(meta.strong.get() + 1); // Cell assessor methods
      Some( // Return Ptr<U> wrapped in Some to indicate success
        // Construct a new Ptr<U> with the newly casted value and shared
        //   meta data
        Ptr { meta: self.meta, value }
      )
    }
  } else {
    None // Return None to indicate a invalid cast
  }
}

