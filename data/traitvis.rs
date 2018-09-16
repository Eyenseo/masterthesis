pub(crate) trait A {}
pub trait B: A {} // can't leak private trait
