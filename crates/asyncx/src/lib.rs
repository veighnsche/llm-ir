use llmir_diag::{Diag, Code};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChanId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tick(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Deadline(pub u64);

pub trait Task {} // TODO
pub trait Chan<T> {} // TODO

pub fn not_available(feature: &'static str) -> Diag {
    Diag::feature_missing(feature)
}

pub fn select_stub() -> Result<(), Diag> {
    Err(not_available("async"))
}
