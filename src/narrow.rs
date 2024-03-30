use super::type_fuckery::{Cons, Here, There};
use super::type_set::TypeSet;

pub(crate) trait Narrow<Target, Index>: TypeSet {
    type Remainder;
}

impl<T, Target, Tail> Narrow<Target, Here> for T
where
    T: TypeSet<TList = Cons<Target, Tail>>,
{
    type Remainder = Tail;
}

impl<T, Head, Tail, Target, TailIndex> Narrow<Target, There<TailIndex>> for T
where
    T: TypeSet<TList = Cons<Head, Tail>>,
    Tail: Narrow<Target, TailIndex>,
{
    type Remainder = Cons<Head, <Tail as Narrow<Target, TailIndex>>::Remainder>;
}

fn _smoke_compile_test() {
    fn can_narrow<TList, Target, Remainder>()
    where
        TList: Narrow<Target, Remainder>,
    {
    }

    type T0 = <(u32, String) as TypeSet>::TList;

    // can_narrow::<T0, u32, (String,)>();
}
