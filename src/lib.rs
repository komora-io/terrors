#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

mod one_of;
mod one_of_to_enum;
mod type_set;

/// Similar to anonymous unions / enums in languages that support type narrowing.
pub use one_of::OneOf;

pub use type_set::{E1, E2, E3, E4, E5, E6, E7, E8, E9};
