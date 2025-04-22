#![cfg_attr(
    feature = "error_provide_feature",
    feature(error_generic_member_access)
)]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#[cfg(doctest)]
pub struct ReadmeDoctests;

mod one_of;
mod one_of_to_enum;
mod type_set;

/// Similar to anonymous unions / enums in languages that support type narrowing.
pub use one_of::OneOf;

pub use type_set::{TypeSet, E1, E2, E3, E4, E5, E6, E7, E8, E9};

/* ------------------------- Helpers ----------------------- */

/// The final element of a type-level Cons list.
#[doc(hidden)]
#[derive(Debug)]
pub enum End {}

impl std::error::Error for End {}

/// A compile-time list of types, similar to other basic functional list structures.
#[doc(hidden)]
#[derive(Debug)]
pub struct Cons<Head, Tail>(core::marker::PhantomData<Head>, Tail);

#[doc(hidden)]
#[derive(Debug)]
pub struct Recurse<Tail>(Tail);
