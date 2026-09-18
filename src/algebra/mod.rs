pub mod frac;

pub use frac::Frac;

#[allow(non_camel_case_types)]
pub type r32 = frac::Frac<i16>;
#[allow(non_camel_case_types)]
pub type r64 = frac::Frac<i32>;
#[allow(non_camel_case_types)]
pub type r128 = frac::Frac<i64>;
