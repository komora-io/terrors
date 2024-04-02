//! Type-level set inclusion and difference, inspired by frunk's approach: https://archive.is/YwDMX

use std::marker::PhantomData;

/* ------------------------- Helpers ----------------------- */

/// The final element of a type-level Cons list.
#[derive(Debug)]
pub enum End {}

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
#[derive(Debug)]
pub struct Cons<Head, Tail>(PhantomData<Head>, Tail);

/* ------------------------- std::error support ----------------------- */
use std::any::Any;
use std::error::Error;
use std::fmt;

impl<Head, Tail> fmt::Display for Cons<Head, Tail>
where
    Head: fmt::Display,
    Tail: fmt::Display,
{
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO handle this reasonably, or make Cons impossible for users to construct.
        Ok(())
    }
}

impl fmt::Display for End {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        unreachable!("Display::fmt called for an End, which is not constructible.")
    }
}

impl<Head, Tail> Error for Cons<Head, Tail>
where
    Head: Error,
    Tail: Error,
{
}

// fold requirements
// * args: Box<dyn Any>
// * params: Cons<Head, Tail>
// * algo: when Box<dyn Any>::is<Head>(), downcast_ref

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

pub trait ErrorFold {
    fn source_fold(any: &Box<dyn Any>) -> Option<&(dyn Error + 'static)>;
}

impl ErrorFold for End {
    fn source_fold(_: &Box<dyn Any>) -> Option<&(dyn Error + 'static)> {
        unreachable!("source_fold called on End");
    }
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
}

/* ------------------------- TypeSet implemented for tuples ----------------------- */

pub trait TypeSet {
    type TList: TupleForm;
    type Enum;
    type EnumRef<'a>
    where
        Self: 'a;
}

impl TypeSet for () {
    type TList = End;
    type Enum = E0;
    type EnumRef<'a> = E0 where Self: 'a;
}

impl<A> TypeSet for (A,) {
    type TList = Cons<A, End>;
    type Enum = E1<A>;
    type EnumRef<'a> = E1<&'a A> where Self: 'a;
}

impl<A, B> TypeSet for (A, B) {
    type TList = Cons<A, Cons<B, End>>;
    type Enum = E2<A, B>;
    type EnumRef<'a> = E2<&'a A, &'a B> where Self: 'a;
}

impl<A, B, C> TypeSet for (A, B, C) {
    type TList = Cons<A, Cons<B, Cons<C, End>>>;
    type Enum = E3<A, B, C>;
    type EnumRef<'a> = E3<&'a A, &'a B, &'a C> where Self: 'a;
}

impl<A, B, C, D> TypeSet for (A, B, C, D) {
    type TList = Cons<A, Cons<B, Cons<C, Cons<D, End>>>>;
    type Enum = E4<A, B, C, D>;
    type EnumRef<'a> = E4<&'a A, &'a B, &'a C, &'a D> where Self: 'a;
}

impl<A, B, C, D, E> TypeSet for (A, B, C, D, E) {
    type TList = Cons<A, Cons<B, Cons<C, Cons<D, Cons<E, End>>>>>;
    type Enum = E5<A, B, C, D, E>;
    type EnumRef<'a> = E5<&'a A, &'a B, &'a C, &'a D, &'a E> where Self: 'a;
}

impl<A, B, C, D, E, F> TypeSet for (A, B, C, D, E, F) {
    type TList = Cons<A, Cons<B, Cons<C, Cons<D, Cons<E, Cons<F, End>>>>>>;
    type Enum = E6<A, B, C, D, E, F>;
    type EnumRef<'a> = E6<&'a A, &'a B, &'a C, &'a D, &'a E, &'a F> where Self: 'a;
}

impl<A, B, C, D, E, F, G> TypeSet for (A, B, C, D, E, F, G) {
    type TList = Cons<A, Cons<B, Cons<C, Cons<D, Cons<E, Cons<F, Cons<G, End>>>>>>>;
    type Enum = E7<A, B, C, D, E, F, G>;
    type EnumRef<'a> = E7<&'a A, &'a B, &'a C, &'a D, &'a E, &'a F, &'a G> where Self: 'a;
}

impl<A, B, C, D, E, F, G, H> TypeSet for (A, B, C, D, E, F, G, H) {
    type TList = Cons<A, Cons<B, Cons<C, Cons<D, Cons<E, Cons<F, Cons<G, Cons<H, End>>>>>>>>;
    type Enum = E8<A, B, C, D, E, F, G, H>;
    type EnumRef<'a> = E8<&'a A, &'a B, &'a C, &'a D, &'a E, &'a F, &'a G, &'a H> where Self: 'a;
}

impl<A, B, C, D, E, F, G, H, I> TypeSet for (A, B, C, D, E, F, G, H, I) {
    type TList =
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
/// The `Index` parameter is either `Here` or `There` depending on
/// whether the trait implementation is a base case or the recursive
/// case.
pub trait Contains<T, Index> {}

/// Base case implementation for when the Cons Head is T.
impl<T, Tail> Contains<T, Here> for Cons<T, Tail> {}

/// Recursive case for when the Cons Tail contains T.
impl<T, Index, Head, Tail> Contains<T, There<Index>> for Cons<Head, Tail> where
    Tail: Contains<T, Index>
{
}

/* ------------------------- Narrow ----------------------- */

/// A trait for pulling a specific type out of a TList at compile-time
/// and having access to the other types as the Remainder.
pub trait Narrow<Target, Index>: TupleForm {
    type Remainder: TupleForm;
}

/// Base case where the search Target is in the Head of the TList.
impl<Target, Tail> Narrow<Target, Here> for Cons<Target, Tail>
where
    Tail: TupleForm,
    Cons<Target, Tail>: TupleForm,
{
    type Remainder = Tail;
}

/// Recursive case where the search Target is in the Tail of the TList.
impl<Head, Tail, Target, Index> Narrow<Target, There<Index>> for Cons<Head, Tail>
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

    type T0 = <(u32, String) as TypeSet>::TList;

    can_narrow::<T0, u32, _, _>();
    can_narrow::<T0, String, Cons<u32, End>, _>();
}

/* ------------------------- SupersetOf ----------------------- */

/// When all types in a TList are present in a second TList
pub trait SupersetOf<Other, Index> {}

/// Base case
impl<T> SupersetOf<End, End> for T {}

/// Recursive case - more complex because we have to reason about the Index itself as a
/// heterogenous list.
impl<SubHead, SubTail, SuperHead, SuperTail, HeadIndex, TailIndex>
    SupersetOf<Cons<SubHead, SubTail>, Cons<HeadIndex, TailIndex>> for Cons<SuperHead, SuperTail>
where
    Cons<SuperHead, SuperTail>: Narrow<SubHead, HeadIndex>,
    <Cons<SuperHead, SuperTail> as Narrow<SubHead, HeadIndex>>::Remainder:
        SupersetOf<SubTail, TailIndex>,
{
}

fn _superset_test() {
    fn is_superset<S1, S2, Index>()
    where
        S1: SupersetOf<S2, Index>,
    {
    }

    type T0 = <(u32,) as TypeSet>::TList;
    type T1A = <(u32, String) as TypeSet>::TList;
    type T1B = <(String, u32) as TypeSet>::TList;
    type T2 = <(String, i32, u32) as TypeSet>::TList;
    type T3 = <(Vec<u8>, Vec<i8>, u32, f32, String, f64, i32) as TypeSet>::TList;

    is_superset::<T0, T0, _>();
    is_superset::<T1A, T1A, _>();
    is_superset::<T1A, T1B, _>();
    is_superset::<T1B, T1A, _>();
    is_superset::<T2, T2, _>();
    is_superset::<T1A, T0, _>();
    is_superset::<T1B, T0, _>();
    is_superset::<T2, T0, _>();
    is_superset::<T2, T1A, _>();
    is_superset::<T2, T1B, _>();
    is_superset::<T3, T1A, _>();
    is_superset::<T3, T1B, _>();
    is_superset::<T3, T0, _>();
    is_superset::<T3, T2, _>();
}
