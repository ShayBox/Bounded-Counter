use num_traits::{bounds::UpperBounded, One};
use std::ops::{AddAssign, RemAssign};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "deref", derive(derive_more::Deref))]
#[cfg_attr(feature = "deref_mut", derive(derive_more::DerefMut))]
pub struct BoundedCounter<I>(pub I);

impl<I> Iterator for BoundedCounter<I>
where
    I: Copy + RemAssign<I> + UpperBounded + AddAssign<I> + One,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.0;

        self.0 %= I::max_value();
        self.0 += I::one();

        Some(i)
    }
}
