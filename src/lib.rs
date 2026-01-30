use num_traits::ops::wrapping::WrappingAdd;
use num_traits::{bounds::UpperBounded, One};
use std::ops::{AddAssign, RemAssign};

/// # Bounded Counter
/// This is a generic type that provides an incremental counter with a type specified upper bound.
/// It increments every time it is iterated until it reaches the upper bound, at which point it resets back to 0.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "constructor", derive(derive_more::Constructor))]
#[cfg_attr(feature = "deref_mut", derive(derive_more::DerefMut))]
#[cfg_attr(feature = "deref", derive(derive_more::Deref))]
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

/// # Wrapped Counter
/// This counter increments on every iteration and wraps on overflow.
/// For unsigned types this wraps to 0 after the max value; for signed types it wraps to the min value.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "constructor", derive(derive_more::Constructor))]
#[cfg_attr(feature = "deref_mut", derive(derive_more::DerefMut))]
#[cfg_attr(feature = "deref", derive(derive_more::Deref))]
pub struct WrappedCounter<I>(pub I);

impl<I> Iterator for WrappedCounter<I>
where
    I: Copy + One + WrappingAdd,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.0;
        self.0 = self.0.wrapping_add(&I::one());
        Some(i)
    }
}
