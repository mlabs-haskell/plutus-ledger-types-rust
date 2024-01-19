use std::{
    collections::BTreeMap,
    iter::{empty, once},
};

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

pub fn union_b_tree_maps_with<const N: usize, K: Clone + Ord, V: Clone, F: Fn(&V, &V) -> V>(
    f: F,
    maps: [&BTreeMap<K, V>; N],
) -> BTreeMap<K, V> {
    maps.into_iter().fold(BTreeMap::new(), |acc, m| {
        m.into_iter().fold(acc, |mut acc, (k, v)| {
            acc.entry(k.clone())
                .and_modify(|va: &mut V| *va = f(va, v))
                .or_insert(v.clone());

            acc
        })
    })
}
