// pretty self-explanatory. Inspired by: https://archive.is/BGgHc and https://archive.is/YwDMX

pub(crate) trait Contains<T, Index> {}
pub(crate) struct End;
pub(crate) struct Cons<Head, Tail>(Head, Tail);

trait HList {}
impl HList for End {}
impl<Head, Tail> HList for Cons<Head, Tail> where Tail: HList {}

pub struct Here;
pub struct There<Index>(Index);

impl<T, Tail> Contains<T, Here> for Cons<T, Tail> {}

impl<T, Index, Head, Tail> Contains<T, There<Index>> for Cons<Head, Tail> where
    Tail: Contains<T, Index>
{
}

// TODO Contains<Cons<T, ...>, Tail> where all T's are contained, for bounding narrowing.
