fn hun(f: &RefCell<F>)
  -> HashMap<i8, i8>
{
  let mut map = HashMap::new();
  map.insert(
    { f.borrow_mut().fun() },
    f.borrow_mut().fun() // BOOM
  );
  map
}
fn nun(f: &RefCell<F>)
  -> HashMap<i8, i8>
{
  let mut map = HashMap::new();
  map.insert(
    {
      let mut f = f.borrow_mut();
      f.fun()
    },
    f.borrow_mut().fun()
  );
  map
}

