use std::{
    collections::BTreeMap,
    iter::{empty, once},
};

pub mod value;

/// Create a container C from one element.
pub fn singleton<T, C>(value: T) -> C
where
    C: FromIterator<T>,
{
    once(value).collect()
}

/// Create an empty container.
pub fn none<T, C>() -> C
where
    C: FromIterator<T>,
{
    empty::<T>().collect()
}

/// Union two BTreeMaps, call f to resolve conflicts if duplicate keys are encountered.
pub fn union_btree_maps_with<K: Clone + Ord, V: Clone, F: Fn(V, V) -> V>(
    f: F,
    l: BTreeMap<K, V>,
    r: BTreeMap<K, V>,
) -> BTreeMap<K, V> {
    r.into_iter().fold(l.clone(), |mut acc, (k, vr)| {
        let v = if let Some((_, vl)) = acc.remove_entry(&k) {
            f(vl, vr)
        } else {
            vr
        };
        acc.insert(k, v);
        acc
    })
}
