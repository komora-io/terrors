//! Type-level set inclusion and difference, inspired by frunk's approach: https://archive.is/YwDMX

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
/// base case. We have to keep wrapping `Index` in a way that is similar
/// in structure to `Cons` because it prevents the compiler from
/// detecting a conflicting implementation in the recursive case.
pub struct There<Index>(PhantomData<Index>);

/// A compile-time list of types, similar to other basic functional list structures.
pub(crate) struct Cons<Head, Tail>(PhantomData<Head>, Tail);

/// A trait that assists with compile-time type set inclusion testing.
/// The `Index` parameter is either `Here` or `There` depending on
/// whether the trait implementation is a base case or the recursive
/// case.
pub(crate) trait Contains<T, Index> {}

/// Base case implementation for when the Cons Head is T.
impl<T, Tail> Contains<T, Here> for Cons<T, Tail> {}

/// Recursive case for when the Cons Tail contains T.
impl<T, Index, Head, Tail> Contains<T, There<Index>> for Cons<Head, Tail> where
    Tail: Contains<T, Index>
{
}

/// A trait for pulling a specific type out of a TList at compile-time
/// and getting its Remainder.
pub(crate) trait Narrow<Target, Index> {
    type Remainder;
}

/// Base case where the search Target is in the Head of the TList.
impl<Target, Tail> Narrow<Target, Here> for Cons<Target, Tail> {
    type Remainder = Tail;
}

/// Recursive case where the search Target is in the Tail of the TList.
impl<Head, Tail, Target, Index> Narrow<Target, There<Index>> for Cons<Head, Tail>
where
    Tail: Narrow<Target, Index>,
{
    type Remainder = Cons<Head, <Tail as Narrow<Target, Index>>::Remainder>;
}

// TODO Contains<Cons<T, ...>, Tail> where all T's are contained, for bounding narrowing.

fn _smoke_compile_test() {
    use super::type_set::TypeSet;

    fn can_narrow<Types, Target, Remainder, Index>()
    where
        Types: Narrow<Target, Index, Remainder = Remainder>,
    {
    }

    type T0 = <(u32, String) as TypeSet>::TList;

    can_narrow::<T0, u32, _, _>();
    can_narrow::<T0, String, Cons<u32, End>, _>();
}
