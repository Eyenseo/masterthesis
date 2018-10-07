fn fun(x: i32) -> Result<DiDi, E> {
  if x == 0 {
    Ok(DiDi)
  } else if x > 7 {
    Err(E)
  } else {
    panic!()
  }
}
fn gun(x: i32) -> Option<DiDi> {
  if x == 42 {
    Some(DiDi)
  } else {
    None
  }
}
fn hun() -> Result<(), E> {
  let x = fun(13)?;
  // ...
  Ok(())
}
fn nun() {
  match fun(2) {
    Ok(didi) => didi
    Err(e) => return e
  }
}
fn pun() {
  if let Some(didi) = gun(42) {
    // ...
  }
}

