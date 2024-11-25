use std::ops::{AddAssign, RemAssign};

use num_traits::{bounds::UpperBounded, One};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BoundedCounter<I>(I);

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
