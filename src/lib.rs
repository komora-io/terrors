#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

mod one_of;
mod type_set;

/// Similar to anonymous unions / enums in languages that support type narrowing.
pub use one_of::OneOf;
