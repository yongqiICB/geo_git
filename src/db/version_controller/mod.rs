#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct VersionId(pub u32);

pub mod r#impl;
pub use self::r#impl::*;
