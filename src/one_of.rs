use std::any::Any;
use std::marker::PhantomData;
use std::ops::Deref;

use crate::type_set::{Cons, Contains, End, Narrow, SupersetOf, TupleForm, TypeSet};

/// `OneOf` is similar to anonymous unions / enums in languages that support type narrowing.
#[derive(Debug)]
pub struct OneOf<E: TypeSet> {
    value: Box<dyn Any>,
    _pd: PhantomData<E>,
}

impl<T> Deref for OneOf<(T,)>
where
    T: 'static,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.value.downcast_ref::<T>().unwrap()
    }
}

impl<T> From<T> for OneOf<(T,)>
where
    T: 'static,
{
    fn from(t: T) -> OneOf<(T,)> {
        OneOf::new(t)
    }
}

impl<E> OneOf<E>
where
    E: TypeSet,
{
    pub fn new<T, Index>(t: T) -> OneOf<E>
    where
        T: Any,
        E::TList: Contains<T, Index>,
    {
        OneOf {
            value: Box::new(t),
            _pd: PhantomData,
        }
    }

    pub fn narrow<Target, Index>(
        self,
    ) -> Result<Target, OneOf<<<E::TList as Narrow<Target, Index>>::Remainder as TupleForm>::Tuple>>
    where
        Target: 'static,
        E::TList: Narrow<Target, Index>,
    {
        if self.value.is::<Target>() {
            Ok(*self.value.downcast::<Target>().unwrap())
        } else {
            Err(OneOf {
                value: self.value,
                _pd: PhantomData,
            })
        }
    }

    pub fn broaden<Other, Index>(self) -> OneOf<Other>
    where
        Other: TypeSet,
        Other::TList: SupersetOf<E::TList, Index>,
    {
        OneOf {
            value: self.value,
            _pd: PhantomData,
        }
    }

    pub fn take<Target>(self) -> Target
    where
        Target: 'static,
        E: TypeSet<TList = Cons<Target, End>>,
    {
        *self.value.downcast::<Target>().unwrap()
    }
}
