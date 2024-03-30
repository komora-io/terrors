use std::any::Any;
use std::marker::PhantomData;

use crate::type_set::{Contains, Narrow, TypeSet};

#[derive(Debug)]
pub struct OneOf<E: TypeSet> {
    value: Box<dyn Any>,
    _pd: PhantomData<E>,
}

impl<T> From<T> for OneOf<(T,)>
where
    T: Any,
{
    fn from(t: T) -> OneOf<(T,)> {
        OneOf::new(t)
    }
}

impl<E> OneOf<E>
where
    E: TypeSet,
{
    pub fn narrow<Target, Remainder, Index>(self) -> Result<Target, OneOf<Remainder>>
    where
        E::TList: Contains<Target, Index>,
        Remainder: TypeSet,
        Target: 'static,
        E::TList: Narrow<Target, Index, Remainder = Remainder::TList>,
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
}
