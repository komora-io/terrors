use super::type_fuckery::{Cons, End};

pub(crate) trait TypeSet {
    type TList;
}

impl TypeSet for () {
    type TList = End;
}

impl<A> TypeSet for (A,) {
    type TList = Cons<A, End>;
}

impl<A, B> TypeSet for (A, B) {
    type TList = Cons<A, Cons<B, End>>;
}

impl<A, B, C> TypeSet for (A, B, C) {
    type TList = Cons<A, Cons<B, Cons<C, End>>>;
}

impl<A, B, C, D> TypeSet for (A, B, C, D) {
    type TList = Cons<A, Cons<B, Cons<C, Cons<D, End>>>>;
}

impl<A, B, C, D, E> TypeSet for (A, B, C, D, E) {
    type TList = Cons<A, Cons<B, Cons<C, Cons<D, Cons<E, End>>>>>;
}

impl<A, B, C, D, E, F> TypeSet for (A, B, C, D, E, F) {
    type TList = Cons<A, Cons<B, Cons<C, Cons<D, Cons<E, Cons<F, End>>>>>>;
}
