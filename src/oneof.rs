use std::any::{Any, TypeId};
use std::marker::PhantomData;

use crate::type_fuckery::{Contains, Here};
use crate::typeset::TypeSet;

#[derive(Debug)]
pub struct OneOf<E: TypeSet> {
    value: Box<dyn Any>,
    _pd: PhantomData<E>,
}

/*
// TODO need to convince compiler that E != T for this to compile
impl<T, E> From<OneOf<T>> for OneOf<E>
where
    T: Any,
    E: TypeSet,
    E::TList: Contains<T, Here>,
{
    fn from(t: T) -> OneOf<(T,)> {
        OneOf::new(t)
    }
}
*/

impl<E> OneOf<E>
where
    E: TypeSet,
{
    pub fn narrow<Target, Remainder, Index>(self) -> Result<Target, OneOf<Remainder>>
    where
        E::TList: Contains<Target, Index>,
        // TODO enforce Remainder being subset of E, as currently it's unconstrained
        Remainder: TypeSet,
        Target: 'static,
    {
        let actual_tid = self.value.type_id();
        if TypeId::of::<Target>() == actual_tid {
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
