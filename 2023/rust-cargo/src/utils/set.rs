use std::collections::{BTreeSet, HashSet};

/// Trait representing data structures that implement the *set* ADT for types T,
/// such as [`HashSet<T>`] or [`BTreeSet<T>`].
pub trait Set<T> {
    fn insert(&mut self, value: T) -> bool;
}

impl<T> Set<T> for HashSet<T> {
    #[allow(unconditional_recursion)]
    fn insert(&mut self, value: T) -> bool {
        HashSet::<T>::insert(self, value)
    }
}

impl<T: Ord> Set<T> for BTreeSet<T> {
    fn insert(&mut self, value: T) -> bool {
        BTreeSet::<T>::insert(self, value)
    }
}
