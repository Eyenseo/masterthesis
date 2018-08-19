fn fun(i: i32) -> Result<DiDi, E> {
    if i == 0 {
        Ok(DiDi)
    } else if val > 7 {
        Err(Boom)
    } else {
        panic!()
    }
}
fn gun() -> Result<(), E> {
    let didi = fun(13)?;
    // ...
    Ok(())
}
fn hun() {
    match fun(2) {
        Ok(didi) => // ...
        Err(e) => // ...
    }
}

