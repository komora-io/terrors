//! Type-level set inclusion and difference, inspired by frunk's approach: <https://archive.is/YwDMX>
use core::any::Any;
use core::fmt;
use std::error::Error;

use crate::{Cons, End, Recurse};

#[cfg(feature = "axum")]
use axum_core::response::IntoResponse;
#[cfg(feature = "axum")]
use axum_core::response::Response;

/* ------------------------- std::error::Error support ----------------------- */

pub trait ErrorFold {
    fn source_fold(any: &Box<dyn Any>) -> Option<&(dyn Error + 'static)>;

    #[cfg(feature = "error_provide")]
    fn provide_fold<'a>(any: &'a Box<dyn Any>, request: &mut std::error::Request<'a>);
}

impl ErrorFold for End {
    fn source_fold(_: &Box<dyn Any>) -> Option<&(dyn Error + 'static)> {
        unreachable!("source_fold called on End");
    }

    #[cfg(feature = "error_provide")]
    fn provide_fold<'a>(_: &Box<dyn Any>, _: &mut std::error::Request<'a>) {
        unreachable!("provide_fold called on End");
    }
}

impl<Head, Tail> Error for Cons<Head, Tail>
where
    Head: Error,
    Tail: Error,
{
}

impl<Head, Tail> ErrorFold for Cons<Head, Tail>
where
    Cons<Head, Tail>: Error,
    Head: 'static + Error,
    Tail: ErrorFold,
{
    fn source_fold(any: &Box<dyn Any>) -> Option<&(dyn Error + 'static)> {
        if let Some(head_ref) = any.downcast_ref::<Head>() {
            head_ref.source()
        } else {
            Tail::source_fold(any)
        }
    }

    #[cfg(feature = "error_provide")]
    fn provide_fold<'a>(any: &'a Box<dyn Any>, request: &mut std::error::Request<'a>) {
        if let Some(head_ref) = any.downcast_ref::<Head>() {
            head_ref.provide(request)
        } else {
            Tail::provide_fold(any, request)
        }
    }
}

/* ------------------------- Axum support ----------------------- */

#[cfg(feature = "axum")]
impl<Head, Tail> IntoResponse for Cons<Head, Tail>
where
    Head: IntoResponse,
    Tail: IntoResponse,
{
    fn into_response(self) -> Response {
        unreachable!("into_response called for Cons which is not constructable")
    }
}

#[cfg(feature = "axum")]
impl IntoResponse for End {
    fn into_response(self) -> Response {
        unreachable!("into_response called for an End, which is not constructible.")
    }
}

#[cfg(feature = "axum")]
pub trait ResponseFold {
    fn response_fold(any: Box<dyn Any>) -> Response;
}

#[cfg(feature = "axum")]
impl ResponseFold for End {
    fn response_fold(_: Box<dyn Any>) -> Response {
        unreachable!("display_fold called on End");
    }
}

#[cfg(feature = "axum")]
impl<Head, Tail> ResponseFold for Cons<Head, Tail>
where
    Cons<Head, Tail>: IntoResponse,
    Head: 'static + IntoResponse,
    Tail: ResponseFold,
{
    fn response_fold(any: Box<dyn Any>) -> Response {
        match any.downcast::<Head>() {
            Ok(head) => head.into_response(),
            Err(tail) => Tail::response_fold(tail),
        }
    }
}

/* ------------------------- Display support ----------------------- */

impl<Head, Tail> fmt::Display for Cons<Head, Tail>
where
    Head: fmt::Display,
    Tail: fmt::Display,
{
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        unreachable!("Display called for Cons which is not constructable")
    }
}

impl fmt::Display for End {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        unreachable!("Display::fmt called for an End, which is not constructible.")
    }
}

pub trait DisplayFold {
    fn display_fold(any: &Box<dyn Any>, formatter: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl DisplayFold for End {
    fn display_fold(_: &Box<dyn Any>, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        unreachable!("display_fold called on End");
    }
}

impl<Head, Tail> DisplayFold for Cons<Head, Tail>
where
    Cons<Head, Tail>: fmt::Display,
    Head: 'static + fmt::Display,
    Tail: DisplayFold,
{
    fn display_fold(any: &Box<dyn Any>, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(head_ref) = any.downcast_ref::<Head>() {
            head_ref.fmt(formatter)
        } else {
            Tail::display_fold(any, formatter)
        }
    }
}

/* ------------------------- Debug support ----------------------- */

pub trait DebugFold {
    fn debug_fold(any: &Box<dyn Any>, formatter: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl DebugFold for End {
    fn debug_fold(_: &Box<dyn Any>, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        unreachable!("debug_fold called on End");
    }
}

impl<Head, Tail> DebugFold for Cons<Head, Tail>
where
    Cons<Head, Tail>: fmt::Debug,
    Head: 'static + fmt::Debug,
    Tail: DebugFold,
{
    fn debug_fold(any: &Box<dyn Any>, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(head_ref) = any.downcast_ref::<Head>() {
            head_ref.fmt(formatter)
        } else {
            Tail::debug_fold(any, formatter)
        }
    }
}

/* ------------------------- Clone support ----------------------- */

pub trait CloneFold {
    fn clone_fold(any: &Box<dyn Any>) -> Box<dyn Any>;
}

impl Clone for End {
    fn clone(&self) -> End {
        unreachable!("clone called for End");
    }
}

impl<Head, Tail> Clone for Cons<Head, Tail>
where
    Head: 'static + Clone,
    Tail: CloneFold,
{
    fn clone(&self) -> Self {
        unreachable!("clone called for Cons which is not constructable");
    }
}

impl CloneFold for End {
    fn clone_fold(_: &Box<dyn Any>) -> Box<dyn Any> {
        unreachable!("clone_fold called on End");
    }
}

impl<Head, Tail> CloneFold for Cons<Head, Tail>
where
    Head: 'static + Clone,
    Tail: CloneFold,
{
    fn clone_fold(any: &Box<dyn Any>) -> Box<dyn Any> {
        if let Some(head_ref) = any.downcast_ref::<Head>() {
            Box::new(head_ref.clone())
        } else {
            Tail::clone_fold(any)
        }
    }
}

fn _clone_test() {
    fn is_clone<T: Clone>() {}

    type T0 = <(String, u32) as TypeSet>::Variants;

    is_clone::<T0>();
}

/* ------------------------- Any::is support ----------------------- */

pub trait IsFold {
    fn is_fold(any: &Box<dyn Any>) -> bool;
}

impl IsFold for End {
    fn is_fold(_: &Box<dyn Any>) -> bool {
        false
    }
}

impl<Head, Tail> IsFold for Cons<Head, Tail>
where
    Head: 'static,
    Tail: IsFold,
{
    fn is_fold(any: &Box<dyn Any>) -> bool {
        if any.is::<Head>() {
            true
        } else {
            Tail::is_fold(any)
        }
    }
}

/* ------------------------- TypeSet implemented for tuples ----------------------- */

pub trait TypeSet {
    type Variants: TupleForm;
    type Enum;
    type EnumRef<'a>
    where
        Self: 'a;
}

impl TypeSet for () {
    type Variants = End;
    type Enum = E0;
    type EnumRef<'a> = E0 where Self: 'a;
}

impl<A> TypeSet for (A,) {
    type Variants = Cons<A, End>;
    type Enum = E1<A>;
    type EnumRef<'a> = E1<&'a A> where Self: 'a;
}

impl<A, B> TypeSet for (A, B) {
    type Variants = Cons<A, Cons<B, End>>;
    type Enum = E2<A, B>;
    type EnumRef<'a> = E2<&'a A, &'a B> where Self: 'a;
}

impl<A, B, C> TypeSet for (A, B, C) {
    type Variants = Cons<A, Cons<B, Cons<C, End>>>;
    type Enum = E3<A, B, C>;
    type EnumRef<'a> = E3<&'a A, &'a B, &'a C> where Self: 'a;
}

impl<A, B, C, D> TypeSet for (A, B, C, D) {
    type Variants = Cons<A, Cons<B, Cons<C, Cons<D, End>>>>;
    type Enum = E4<A, B, C, D>;
    type EnumRef<'a> = E4<&'a A, &'a B, &'a C, &'a D> where Self: 'a;
}

impl<A, B, C, D, E> TypeSet for (A, B, C, D, E) {
    type Variants = Cons<A, Cons<B, Cons<C, Cons<D, Cons<E, End>>>>>;
    type Enum = E5<A, B, C, D, E>;
    type EnumRef<'a> = E5<&'a A, &'a B, &'a C, &'a D, &'a E> where Self: 'a;
}

impl<A, B, C, D, E, F> TypeSet for (A, B, C, D, E, F) {
    type Variants = Cons<A, Cons<B, Cons<C, Cons<D, Cons<E, Cons<F, End>>>>>>;
    type Enum = E6<A, B, C, D, E, F>;
    type EnumRef<'a> = E6<&'a A, &'a B, &'a C, &'a D, &'a E, &'a F> where Self: 'a;
}

impl<A, B, C, D, E, F, G> TypeSet for (A, B, C, D, E, F, G) {
    type Variants = Cons<A, Cons<B, Cons<C, Cons<D, Cons<E, Cons<F, Cons<G, End>>>>>>>;
    type Enum = E7<A, B, C, D, E, F, G>;
    type EnumRef<'a> = E7<&'a A, &'a B, &'a C, &'a D, &'a E, &'a F, &'a G> where Self: 'a;
}

impl<A, B, C, D, E, F, G, H> TypeSet for (A, B, C, D, E, F, G, H) {
    type Variants = Cons<A, Cons<B, Cons<C, Cons<D, Cons<E, Cons<F, Cons<G, Cons<H, End>>>>>>>>;
    type Enum = E8<A, B, C, D, E, F, G, H>;
    type EnumRef<'a> = E8<&'a A, &'a B, &'a C, &'a D, &'a E, &'a F, &'a G, &'a H> where Self: 'a;
}

impl<A, B, C, D, E, F, G, H, I> TypeSet for (A, B, C, D, E, F, G, H, I) {
    type Variants =
        Cons<A, Cons<B, Cons<C, Cons<D, Cons<E, Cons<F, Cons<G, Cons<H, Cons<I, End>>>>>>>>>;
    type Enum = E9<A, B, C, D, E, F, G, H, I>;
    type EnumRef<'a> = E9<&'a A, &'a B, &'a C, &'a D, &'a E, &'a F, &'a G, &'a H, &'a I> where Self: 'a;
}

/* ------------------------- TupleForm implemented for TypeSet ----------------------- */

pub trait TupleForm {
    type Tuple: TypeSet;
}

impl TupleForm for End {
    type Tuple = ();
}

impl<A> TupleForm for Cons<A, End> {
    type Tuple = (A,);
}

impl<A, B> TupleForm for Cons<A, Cons<B, End>> {
    type Tuple = (A, B);
}

impl<A, B, C> TupleForm for Cons<A, Cons<B, Cons<C, End>>> {
    type Tuple = (A, B, C);
}

impl<A, B, C, D> TupleForm for Cons<A, Cons<B, Cons<C, Cons<D, End>>>> {
    type Tuple = (A, B, C, D);
}

impl<A, B, C, D, E> TupleForm for Cons<A, Cons<B, Cons<C, Cons<D, Cons<E, End>>>>> {
    type Tuple = (A, B, C, D, E);
}

impl<A, B, C, D, E, F> TupleForm for Cons<A, Cons<B, Cons<C, Cons<D, Cons<E, Cons<F, End>>>>>> {
    type Tuple = (A, B, C, D, E, F);
}

impl<A, B, C, D, E, F, G> TupleForm
    for Cons<A, Cons<B, Cons<C, Cons<D, Cons<E, Cons<F, Cons<G, End>>>>>>>
{
    type Tuple = (A, B, C, D, E, F, G);
}

impl<A, B, C, D, E, F, G, H> TupleForm
    for Cons<A, Cons<B, Cons<C, Cons<D, Cons<E, Cons<F, Cons<G, Cons<H, End>>>>>>>>
{
    type Tuple = (A, B, C, D, E, F, G, H);
}

impl<A, B, C, D, E, F, G, H, I> TupleForm
    for Cons<A, Cons<B, Cons<C, Cons<D, Cons<E, Cons<F, Cons<G, Cons<H, Cons<I, End>>>>>>>>>
{
    type Tuple = (A, B, C, D, E, F, G, H, I);
}

/* ------------------------- Lifted ----------------------- */

pub enum E0 {}
pub enum E1<A> {
    A(A),
}
impl<A> From<A> for E1<A> {
    fn from(a: A) -> E1<A> {
        E1::A(a)
    }
}
pub enum E2<A, B> {
    A(A),
    B(B),
}
pub enum E3<A, B, C> {
    A(A),
    B(B),
    C(C),
}
pub enum E4<A, B, C, D> {
    A(A),
    B(B),
    C(C),
    D(D),
}
pub enum E5<A, B, C, D, E> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
}
pub enum E6<A, B, C, D, E, F> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
}
pub enum E7<A, B, C, D, E, F, G> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
    G(G),
}
pub enum E8<A, B, C, D, E, F, G, H> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
    G(G),
    H(H),
}
pub enum E9<A, B, C, D, E, F, G, H, I> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
    G(G),
    H(H),
    I(I),
}

/* ------------------------- Contains ----------------------- */

/// A trait that assists with compile-time type set inclusion testing.
/// The `Index` parameter is either `End` or `Cons<...>` depending on
/// whether the trait implementation is a base case or the recursive
/// case.
pub trait Contains<T, Index> {}

/// Base case implementation for when the Cons Head is T.
impl<T, Tail> Contains<T, End> for Cons<T, Tail> {}

/// Recursive case for when the Cons Tail contains T.
impl<T, Index, Head, Tail> Contains<T, Cons<Index, ()>> for Cons<Head, Tail> where
    Tail: Contains<T, Index>
{
}

/* ------------------------- Narrow ----------------------- */

/// A trait for pulling a specific type out of a Variants at compile-time
/// and having access to the other types as the Remainder.
pub trait Narrow<Target, Index>: TupleForm {
    type Remainder: TupleForm;
}

/// Base case where the search Target is in the Head of the Variants.
impl<Target, Tail> Narrow<Target, End> for Cons<Target, Tail>
where
    Tail: TupleForm,
    Cons<Target, Tail>: TupleForm,
{
    type Remainder = Tail;
}

/// Recursive case where the search Target is in the Tail of the Variants.
impl<Head, Tail, Target, Index> Narrow<Target, Recurse<Index>> for Cons<Head, Tail>
where
    Tail: Narrow<Target, Index>,
    Tail: TupleForm,
    Cons<Head, Tail>: TupleForm,
    Cons<Head, <Tail as Narrow<Target, Index>>::Remainder>: TupleForm,
{
    type Remainder = Cons<Head, <Tail as Narrow<Target, Index>>::Remainder>;
}

fn _narrow_test() {
    fn can_narrow<Types, Target, Remainder, Index>()
    where
        Types: Narrow<Target, Index, Remainder = Remainder>,
    {
    }

    type T0 = <(u32, String) as TypeSet>::Variants;

    can_narrow::<T0, u32, _, _>();
    can_narrow::<T0, String, Cons<u32, End>, _>();
}

/* ------------------------- SupersetOf ----------------------- */

/// When all types in a Variants are present in a second Variants
pub trait SupersetOf<Other, Index> {
    type Remainder: TupleForm;
}

/// Base case
impl<T: TupleForm> SupersetOf<End, End> for T {
    type Remainder = T;
}

/// Recursive case - more complex because we have to reason about the Index itself as a
/// heterogenous list.
impl<SubHead, SubTail, SuperHead, SuperTail, HeadIndex, TailIndex>
    SupersetOf<Cons<SubHead, SubTail>, Cons<HeadIndex, TailIndex>> for Cons<SuperHead, SuperTail>
where
    Cons<SuperHead, SuperTail>: Narrow<SubHead, HeadIndex>,
    <Cons<SuperHead, SuperTail> as Narrow<SubHead, HeadIndex>>::Remainder:
        SupersetOf<SubTail, TailIndex>,
{
    type Remainder =
        <<Cons<SuperHead, SuperTail> as Narrow<SubHead, HeadIndex>>::Remainder as SupersetOf<
            SubTail,
            TailIndex,
        >>::Remainder;
}

fn _superset_test() {
    fn is_superset<S1, S2, Remainder, Index>()
    where
        S1: SupersetOf<S2, Index, Remainder = Remainder>,
    {
    }

    type T0 = <(u32,) as TypeSet>::Variants;
    type T1A = <(u32, String) as TypeSet>::Variants;
    type T1B = <(String, u32) as TypeSet>::Variants;
    type T2 = <(String, i32, u32) as TypeSet>::Variants;
    type T3 = <(Vec<u8>, Vec<i8>, u32, f32, String, f64, i32) as TypeSet>::Variants;

    is_superset::<T0, T0, _, _>();
    is_superset::<T1A, T1A, _, _>();
    is_superset::<T1A, T1B, _, _>();
    is_superset::<T1B, T1A, _, _>();
    is_superset::<T2, T2, _, _>();
    is_superset::<T1A, T0, _, _>();
    is_superset::<T1B, T0, _, _>();
    is_superset::<T2, T0, <(String, i32) as TypeSet>::Variants, _>();
    is_superset::<T2, T1A, <(i32,) as TypeSet>::Variants, _>();
    is_superset::<T2, T1B, <(i32,) as TypeSet>::Variants, _>();
    is_superset::<T3, T1A, <(Vec<u8>, Vec<i8>, f32, f64, i32) as TypeSet>::Variants, _>();
    is_superset::<T3, T1B, _, _>();
    is_superset::<T3, T0, _, _>();
    is_superset::<T3, T2, _, _>();

    type T5sup = <(u8, u16, u32, u64, u128) as TypeSet>::Variants;
    type T5sub = <(u8, u128) as TypeSet>::Variants;
    type T5rem = <(u16, u32, u64) as TypeSet>::Variants;

    is_superset::<T5sup, T5sub, T5rem, _>();
}
