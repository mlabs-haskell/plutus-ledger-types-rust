use std::iter::{empty, once};

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
