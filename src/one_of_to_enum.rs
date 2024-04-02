use super::{OneOf, E1, E2, E3, E4, E5, E6, E7, E8, E9};

/* ------------------------- Enum conversions ----------------------- */

impl<A> From<OneOf<(A,)>> for E1<A>
where
    A: 'static,
{
    fn from(one_of: OneOf<(A,)>) -> Self {
        E1::A(*one_of.value.downcast().unwrap())
    }
}

impl<'a, A> From<&'a OneOf<(A,)>> for E1<&'a A>
where
    A: 'static,
{
    fn from(one_of: &'a OneOf<(A,)>) -> Self {
        E1::A(one_of.value.downcast_ref().unwrap())
    }
}

impl<A, B> From<OneOf<(A, B)>> for E2<A, B>
where
    A: 'static,
    B: 'static,
{
    fn from(one_of: OneOf<(A, B)>) -> Self {
        if one_of.value.is::<A>() {
            E2::A(*one_of.value.downcast().unwrap())
        } else {
            E2::B(*one_of.value.downcast().unwrap())
        }
    }
}

impl<'a, A, B> From<&'a OneOf<(A, B)>> for E2<&'a A, &'a B>
where
    A: 'static,
    B: 'static,
{
    fn from(one_of: &'a OneOf<(A, B)>) -> Self {
        if one_of.value.is::<A>() {
            E2::A(one_of.value.downcast_ref().unwrap())
        } else {
            E2::B(one_of.value.downcast_ref().unwrap())
        }
    }
}

impl<A, B, C> From<OneOf<(A, B, C)>> for E3<A, B, C>
where
    A: 'static,
    B: 'static,
    C: 'static,
{
    fn from(one_of: OneOf<(A, B, C)>) -> Self {
        if one_of.value.is::<A>() {
            E3::A(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<B>() {
            E3::B(*one_of.value.downcast().unwrap())
        } else {
            E3::C(*one_of.value.downcast().unwrap())
        }
    }
}

impl<'a, A, B, C> From<&'a OneOf<(A, B, C)>> for E3<&'a A, &'a B, &'a C>
where
    A: 'static,
    B: 'static,
    C: 'static,
{
    fn from(one_of: &'a OneOf<(A, B, C)>) -> Self {
        if one_of.value.is::<A>() {
            E3::A(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<B>() {
            E3::B(one_of.value.downcast_ref().unwrap())
        } else {
            E3::C(one_of.value.downcast_ref().unwrap())
        }
    }
}

impl<A, B, C, D> From<OneOf<(A, B, C, D)>> for E4<A, B, C, D>
where
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
{
    fn from(one_of: OneOf<(A, B, C, D)>) -> Self {
        if one_of.value.is::<A>() {
            E4::A(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<B>() {
            E4::B(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<C>() {
            E4::C(*one_of.value.downcast().unwrap())
        } else {
            E4::D(*one_of.value.downcast().unwrap())
        }
    }
}

impl<'a, A, B, C, D> From<&'a OneOf<(A, B, C, D)>> for E4<&'a A, &'a B, &'a C, &'a D>
where
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
{
    fn from(one_of: &'a OneOf<(A, B, C, D)>) -> Self {
        if one_of.value.is::<A>() {
            E4::A(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<B>() {
            E4::B(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<C>() {
            E4::C(one_of.value.downcast_ref().unwrap())
        } else {
            E4::D(one_of.value.downcast_ref().unwrap())
        }
    }
}

impl<A, B, C, D, E> From<OneOf<(A, B, C, D, E)>> for E5<A, B, C, D, E>
where
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
{
    fn from(one_of: OneOf<(A, B, C, D, E)>) -> Self {
        if one_of.value.is::<A>() {
            E5::A(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<B>() {
            E5::B(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<C>() {
            E5::C(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<D>() {
            E5::D(*one_of.value.downcast().unwrap())
        } else {
            E5::E(*one_of.value.downcast().unwrap())
        }
    }
}

impl<'a, A, B, C, D, E> From<&'a OneOf<(A, B, C, D, E)>> for E5<&'a A, &'a B, &'a C, &'a D, &'a E>
where
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
{
    fn from(one_of: &'a OneOf<(A, B, C, D, E)>) -> Self {
        if one_of.value.is::<A>() {
            E5::A(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<B>() {
            E5::B(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<C>() {
            E5::C(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<D>() {
            E5::D(one_of.value.downcast_ref().unwrap())
        } else {
            E5::E(one_of.value.downcast_ref().unwrap())
        }
    }
}

impl<A, B, C, D, E, F> From<OneOf<(A, B, C, D, E, F)>> for E6<A, B, C, D, E, F>
where
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
{
    fn from(one_of: OneOf<(A, B, C, D, E, F)>) -> Self {
        if one_of.value.is::<A>() {
            E6::A(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<B>() {
            E6::B(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<C>() {
            E6::C(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<D>() {
            E6::D(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<E>() {
            E6::E(*one_of.value.downcast().unwrap())
        } else {
            E6::F(*one_of.value.downcast().unwrap())
        }
    }
}

impl<'a, A, B, C, D, E, F> From<&'a OneOf<(A, B, C, D, E, F)>>
    for E6<&'a A, &'a B, &'a C, &'a D, &'a E, &'a F>
where
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
{
    fn from(one_of: &'a OneOf<(A, B, C, D, E, F)>) -> Self {
        if one_of.value.is::<A>() {
            E6::A(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<B>() {
            E6::B(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<C>() {
            E6::C(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<D>() {
            E6::D(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<E>() {
            E6::E(one_of.value.downcast_ref().unwrap())
        } else {
            E6::F(one_of.value.downcast_ref().unwrap())
        }
    }
}

impl<A, B, C, D, E, F, G> From<OneOf<(A, B, C, D, E, F, G)>> for E7<A, B, C, D, E, F, G>
where
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
    G: 'static,
{
    fn from(one_of: OneOf<(A, B, C, D, E, F, G)>) -> Self {
        if one_of.value.is::<A>() {
            E7::A(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<B>() {
            E7::B(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<C>() {
            E7::C(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<D>() {
            E7::D(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<E>() {
            E7::E(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<F>() {
            E7::F(*one_of.value.downcast().unwrap())
        } else {
            E7::G(*one_of.value.downcast().unwrap())
        }
    }
}

impl<'a, A, B, C, D, E, F, G> From<&'a OneOf<(A, B, C, D, E, F, G)>>
    for E7<&'a A, &'a B, &'a C, &'a D, &'a E, &'a F, &'a G>
where
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
    G: 'static,
{
    fn from(one_of: &'a OneOf<(A, B, C, D, E, F, G)>) -> Self {
        if one_of.value.is::<A>() {
            E7::A(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<B>() {
            E7::B(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<C>() {
            E7::C(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<D>() {
            E7::D(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<E>() {
            E7::E(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<F>() {
            E7::F(one_of.value.downcast_ref().unwrap())
        } else {
            E7::G(one_of.value.downcast_ref().unwrap())
        }
    }
}

impl<A, B, C, D, E, F, G, H> From<OneOf<(A, B, C, D, E, F, G, H)>> for E8<A, B, C, D, E, F, G, H>
where
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
    G: 'static,
    H: 'static,
{
    fn from(one_of: OneOf<(A, B, C, D, E, F, G, H)>) -> Self {
        if one_of.value.is::<A>() {
            E8::A(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<B>() {
            E8::B(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<C>() {
            E8::C(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<D>() {
            E8::D(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<E>() {
            E8::E(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<F>() {
            E8::F(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<G>() {
            E8::G(*one_of.value.downcast().unwrap())
        } else {
            E8::H(*one_of.value.downcast().unwrap())
        }
    }
}

impl<'a, A, B, C, D, E, F, G, H> From<&'a OneOf<(A, B, C, D, E, F, G, H)>>
    for E8<&'a A, &'a B, &'a C, &'a D, &'a E, &'a F, &'a G, &'a H>
where
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
    G: 'static,
    H: 'static,
{
    fn from(one_of: &'a OneOf<(A, B, C, D, E, F, G, H)>) -> Self {
        if one_of.value.is::<A>() {
            E8::A(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<B>() {
            E8::B(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<C>() {
            E8::C(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<D>() {
            E8::D(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<E>() {
            E8::E(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<F>() {
            E8::F(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<G>() {
            E8::G(one_of.value.downcast_ref().unwrap())
        } else {
            E8::H(one_of.value.downcast_ref().unwrap())
        }
    }
}

impl<A, B, C, D, E, F, G, H, I> From<OneOf<(A, B, C, D, E, F, G, H, I)>>
    for E9<A, B, C, D, E, F, G, H, I>
where
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
    G: 'static,
    H: 'static,
    I: 'static,
{
    fn from(one_of: OneOf<(A, B, C, D, E, F, G, H, I)>) -> Self {
        if one_of.value.is::<A>() {
            E9::A(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<B>() {
            E9::B(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<C>() {
            E9::C(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<D>() {
            E9::D(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<E>() {
            E9::E(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<F>() {
            E9::F(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<G>() {
            E9::G(*one_of.value.downcast().unwrap())
        } else if one_of.value.is::<H>() {
            E9::H(*one_of.value.downcast().unwrap())
        } else {
            E9::I(*one_of.value.downcast().unwrap())
        }
    }
}

impl<'a, A, B, C, D, E, F, G, H, I> From<&'a OneOf<(A, B, C, D, E, F, G, H, I)>>
    for E9<&'a A, &'a B, &'a C, &'a D, &'a E, &'a F, &'a G, &'a H, &'a I>
where
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
    G: 'static,
    H: 'static,
    I: 'static,
{
    fn from(one_of: &'a OneOf<(A, B, C, D, E, F, G, H, I)>) -> Self {
        if one_of.value.is::<A>() {
            E9::A(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<B>() {
            E9::B(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<C>() {
            E9::C(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<D>() {
            E9::D(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<E>() {
            E9::E(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<F>() {
            E9::F(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<G>() {
            E9::G(one_of.value.downcast_ref().unwrap())
        } else if one_of.value.is::<H>() {
            E9::H(one_of.value.downcast_ref().unwrap())
        } else {
            E9::I(one_of.value.downcast_ref().unwrap())
        }
    }
}
