use core::any::Any;
use core::fmt;
use core::marker::PhantomData;
use core::ops::Deref;
use std::error::Error;

#[cfg(feature = "axum")]
use crate::type_set::ResponseFold;
use crate::type_set::{
    CloneFold, Contains, DebugFold, DisplayFold, ErrorFold, IsFold, Narrow, SupersetOf, TupleForm,
    TypeSet,
};
#[cfg(feature = "axum")]
use axum_core::response::IntoResponse;

#[cfg(feature = "axum")]
use axum_core::response::Response;

use crate::{Cons, End};

/* ------------------------- OneOf ----------------------- */

/// `OneOf` is an open sum type. It differs from an enum
/// in that you do not need to define any actual new type
/// in order to hold some specific combination of variants,
/// but rather you simply describe the OneOf as holding
/// one value out of several specific possibilities,
/// defined by using a tuple of those possible variants
/// as the generic parameter for the `OneOf`.
///
/// For example, a `OneOf<(String, u32)>` contains either
/// a `String` or a `u32`. The value over a simple `Result`
/// or other traditional enum starts to become apparent in larger
/// codebases where error handling needs to occur in
/// different places for different errors. `OneOf` allows
/// you to quickly specify a function's return value as
/// involving a precise subset of errors that the caller
/// can clearly reason about.
pub struct OneOf<E: TypeSet> {
    pub(crate) value: Box<dyn Any>,
    _pd: PhantomData<E>,
}

fn _send_sync_error_assert() {
    use std::io;

    fn is_send<T: Send>(_: &T) {}
    fn is_sync<T: Sync>(_: &T) {}
    fn is_error<T: Error>(_: &T) {}

    let o: OneOf<(io::Error,)> = OneOf::new(io::Error::new(io::ErrorKind::Other, "yooo"));
    is_send(&o);
    is_sync(&o);
    is_error(&o);
}

unsafe impl<T> Send for OneOf<T> where T: TypeSet + Send {}
unsafe impl<T> Sync for OneOf<T> where T: TypeSet + Sync {}

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

impl<E> Clone for OneOf<E>
where
    E: TypeSet,
    E::Variants: Clone + CloneFold,
{
    fn clone(&self) -> Self {
        let value = E::Variants::clone_fold(&self.value);

        OneOf {
            value,
            _pd: PhantomData,
        }
    }
}
impl<E> fmt::Debug for OneOf<E>
where
    E: TypeSet,
    E::Variants: fmt::Debug + DebugFold,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        E::Variants::debug_fold(&self.value, formatter)
    }
}

impl<E> fmt::Display for OneOf<E>
where
    E: TypeSet,
    E::Variants: fmt::Display + DisplayFold,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        E::Variants::display_fold(&self.value, formatter)
    }
}

#[cfg(feature = "axum")]
impl<E> IntoResponse for OneOf<E>
where
    E: TypeSet,
    E::Variants: IntoResponse + ResponseFold,
{
    fn into_response(self) -> Response {
        E::Variants::response_fold(self.value)
    }
}

impl<E> Error for OneOf<E>
where
    E: TypeSet,
    E::Variants: Error + DebugFold + DisplayFold + ErrorFold,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        E::Variants::source_fold(&self.value)
    }
}

impl<E> OneOf<E>
where
    E: TypeSet,
{
    /// Create a new `OneOf`.
    pub fn new<T, Index>(t: T) -> OneOf<E>
    where
        T: Any,
        E::Variants: Contains<T, Index>,
    {
        OneOf {
            value: Box::new(t),
            _pd: PhantomData,
        }
    }

    /// Attempt to downcast the `OneOf` into a specific type, and
    /// if that fails, return a `OneOf` which does not contain that
    /// type as one of its possible variants.
    pub fn narrow<Target, Index>(
        self,
    ) -> Result<
        Target,
        OneOf<<<E::Variants as Narrow<Target, Index>>::Remainder as TupleForm>::Tuple>,
    >
    where
        Target: 'static,
        E::Variants: Narrow<Target, Index>,
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

    /// Turns the `OneOf` into a `OneOf` with a set of variants
    /// which is a superset of the current one. This may also be
    /// the same set of variants, but in a different order.
    pub fn broaden<Other, Index>(self) -> OneOf<Other>
    where
        Other: TypeSet,
        Other::Variants: SupersetOf<E::Variants, Index>,
    {
        OneOf {
            value: self.value,
            _pd: PhantomData,
        }
    }

    /// Attempt to split a subset of variants out of the `OneOf`,
    /// returning the remainder of possible variants if the value
    /// does not have one of the `TargetList` types.
    pub fn subset<TargetList, Index>(
        self,
    ) -> Result<
        OneOf<TargetList>,
        OneOf<<<E::Variants as SupersetOf<TargetList::Variants, Index>>::Remainder as TupleForm>::Tuple>,
    >
    where
        TargetList: TypeSet,
        E::Variants: IsFold + SupersetOf<TargetList::Variants, Index>,
    {
        if E::Variants::is_fold(&self.value) {
            Ok(OneOf {
                value: self.value,
                _pd: PhantomData,
            })
        } else {
            Err(OneOf {
                value: self.value,
                _pd: PhantomData,
            })
        }
    }

    /// For a `OneOf` with a single variant, return
    /// the contained value.
    pub fn take<Target>(self) -> Target
    where
        Target: 'static,
        E: TypeSet<Variants = Cons<Target, End>>,
    {
        *self.value.downcast::<Target>().unwrap()
    }

    /// Convert the `OneOf` to an owned enum for
    /// use in pattern matching etc...
    pub fn to_enum(self) -> E::Enum
    where
        E::Enum: From<Self>,
    {
        E::Enum::from(self)
    }

    /// Borrow the enum as an enum for use in
    /// pattern matching etc...
    pub fn as_enum<'a>(&'a self) -> E::EnumRef<'a>
    where
        E::EnumRef<'a>: From<&'a Self>,
    {
        E::EnumRef::from(&self)
    }
}
