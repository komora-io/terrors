// pretty self-explanatory. Inspired by frunk's approach: https://archive.is/YwDMX
use std::marker::PhantomData;

/// The final element of a type-level Cons list.
pub(crate) enum End {}

/// A type that signals to the compiler that we focus non-recursively
/// on a particular element, while importantly differentiating it from
/// `There` which allows the compiler to understand that there are no
/// conflicting trait implementations that overlap.
pub enum Here {}

/// The complement to `Here`, this allows the compiler to reason about
/// the recursive case of trait resolution in contrast to the `Here`
/// base case.
pub struct There<Index>(PhantomData<Index>);

/// A compile-time list of types, similar to other basic functional list structures.
pub(crate) struct Cons<Head, Tail>(PhantomData<Head>, Tail);

/// A trait that assists with compile-time type set inclusion testing.
/// The `Index` parameter is either `Here` or `There` depending on
/// whether the trait implementation is a base case or the recursive
/// case.
pub(crate) trait Contains<T, Index> {}

/// Base case implementation.
impl<T, Tail> Contains<T, Here> for Cons<T, Tail> {}

/// Recursive case.
impl<T, Index, Head, Tail> Contains<T, There<Index>> for Cons<Head, Tail> where
    Tail: Contains<T, Index>
{
}

// TODO Contains<Cons<T, ...>, Tail> where all T's are contained, for bounding narrowing.
