if let Some(sup_pool) = pool.get_super() { // Option<Weak> -> Weak
  let mut sup_pool = sup_pool.upgrade().unwrap(); // Weak->Option<Rc>->Rc
  let mut sup_pool = sup_pool.borrow_mut(); // Rc<RefCell -> RefMut
  let mut sup_pool = sup_pool.pool_mut(); // RefMut<UsrPool> -> Pool
  // ...
}

