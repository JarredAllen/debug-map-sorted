#![doc = include_str!("../README.md")]

use itertools::Itertools;
use std::{collections::HashMap, fmt::Debug, hash::Hash};

/// A wrapper for [`HashMap`] which sorts itself for `Debug` output
///
/// You probably want to use a [`BTreeMap`](std::collections::BTreeMap) if you want your
/// data to be sorted, but if outputting is rare (or is only included transiently for
/// debugging purposes), it may make more sense to use this trait instead of changing the
/// underlying type.
///
/// Also see [`SortedOutputExt`] for a syntactically cleaner method of constructing this type.
pub struct SortedHashMapDebugOutput<'a, K, V>(pub &'a HashMap<K, V>);

impl<'a, K, V> Debug for SortedHashMapDebugOutput<'a, K, V>
where
    K: Ord + Debug + Eq + Hash,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entries(self.0.keys().sorted().map(|k| (k, &self.0[k])))
            .finish()
    }
}

/// An extension trait for easier production of [`SortedHashMapDebugOutput`] values
///
/// This trait is generic because it may be extended in future versions of the crate, but for
/// now it only applies to [`HashMap`].
pub trait SortedOutputExt {
    /// The target type
    type Sorted<'a>
    where
        Self: 'a;

    /// A method which borrows `self` as the given target
    fn sorted_debug(&self) -> Self::Sorted<'_>;
}

impl<K, V> SortedOutputExt for HashMap<K, V> {
    type Sorted<'a> = SortedHashMapDebugOutput<'a, K, V> where Self: 'a;
    fn sorted_debug(&self) -> Self::Sorted<'_> {
        SortedHashMapDebugOutput(self)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use super::SortedHashMapDebugOutput;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    /// Test by comparing the output against the debug of a [`BTreeMap`]
    fn test_output_alphebatized(data: HashMap<isize, isize>) -> bool {
        format!("{:?}", SortedHashMapDebugOutput(&data))
            == format!(
                "{:?}",
                data.iter()
                    .map(|(&k, &v)| (k, v))
                    .collect::<BTreeMap<_, _>>()
            )
    }
}
