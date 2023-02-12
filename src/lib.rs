use std::marker::PhantomData;
use std::slice;

/// Like IntoIterator, but not consume current object, return a readonly iterator.
pub trait IterView<'a> {
    type Item: 'a;
    type Iter: Iterator<Item = Self::Item>;
    fn iter(&'a self) -> Self::Iter;
}

impl<'a, T: 'a + IterView<'a>> IterView<'a> for &'a T {
    type Item = T::Item;
    type Iter = T::Iter;
    fn iter(&'a self) -> Self::Iter {
        (*self).iter()
    }
}

impl<'a, T: 'a> IterView<'a> for Vec<T> {
    type Item = &'a T;
    type Iter = slice::Iter<'a, T>;
    fn iter(&'a self) -> Self::Iter {
        self[..].iter()
    }
}

impl<'a, T: 'a> IterView<'a> for [T] {
    type Item = &'a T;
    type Iter = slice::Iter<'a, T>;
    fn iter(&'a self) -> Self::Iter {
        self.iter()
    }
}

impl<'a, T: 'a> IterView<'a> for Option<T> {
    type Item = &'a T;
    type Iter = std::option::Iter<'a, T>;
    fn iter(&'a self) -> Self::Iter {
        self.iter()
    }
}

impl<'a, T: 'a, E: 'a> IterView<'a> for Result<T, E> {
    type Item = &'a T;
    type Iter = std::result::Iter<'a, T>;
    fn iter(&'a self) -> Self::Iter {
        self.iter()
    }
}

impl<'a, T> IterView<'a> for Box<T>
where
    T: IterView<'a>,
{
    type Item = T::Item;
    type Iter = T::Iter;
    fn iter(&'a self) -> Self::Iter {
        self.as_ref().iter()
    }
}

impl<'a, K, V> IterView<'a> for std::collections::HashMap<K, V>
where
    K: Eq + std::hash::Hash + 'a,
    V: 'a,
{
    type Item = (&'a K, &'a V);
    type Iter = std::collections::hash_map::Iter<'a, K, V>;
    fn iter(&'a self) -> Self::Iter {
        self.iter()
    }
}

impl<'a, T> IterView<'a> for std::collections::LinkedList<T>
where
    T: 'a,
{
    type Item = &'a T;
    type Iter = std::collections::linked_list::Iter<'a, T>;
    fn iter(&'a self) -> Self::Iter {
        self.iter()
    }
}

impl<'a, T> IterView<'a> for std::collections::BinaryHeap<T>
where
    T: 'a,
{
    type Item = &'a T;
    type Iter = std::collections::binary_heap::Iter<'a, T>;
    fn iter(&'a self) -> Self::Iter {
        self.iter()
    }
}

impl<'a, T> IterView<'a> for std::collections::VecDeque<T>
where
    T: 'a,
{
    type Item = &'a T;
    type Iter = std::collections::vec_deque::Iter<'a, T>;
    fn iter(&'a self) -> Self::Iter {
        self.iter()
    }
}

impl<'a, T> IterView<'a> for std::collections::HashSet<T>
where
    T: Eq + std::hash::Hash + 'a,
{
    type Item = &'a T;
    type Iter = std::collections::hash_set::Iter<'a, T>;
    fn iter(&'a self) -> Self::Iter {
        self.iter()
    }
}

/// Using a function to iter view a value.
pub fn iter<T, O, F, I>(o: &O, f: F) -> FuncIterView<T, O, F, I> {
    FuncIterView {
        f,
        o,
        t: PhantomData,
        i: PhantomData,
    }
}

pub struct FuncIterView<'a, T, O, F, I> {
    f: F,
    o: &'a O,
    t: PhantomData<T>,
    i: PhantomData<I>,
}

impl<'a, T, O, F, I> IterView<'a> for FuncIterView<'a, T, O, F, I>
where
    T: 'a,
    F: Fn(&O) -> I,
    I: Iterator<Item = T> + 'a,
{
    type Item = T;
    type Iter = I;
    fn iter(&self) -> Self::Iter {
        (self.f)(self.o)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn iter_view<'a, T: IterView<'a>>(o: &'a T) -> T::Iter {
        o.iter()
    }

    #[test]
    fn iter_vec() {
        let v = vec![1, 2, 3];
        let mut iter = iter_view(&v);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_option() {
        let mut v = Some(1);
        let mut iter = iter_view(&v);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        v = None;
        let mut iter = iter_view(&v);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_result() {
        let v: std::io::Result<i32> = Ok(1i32);
        let mut iter = iter_view(&v);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        let v: Result<(), &str> = Err("foo");
        let mut iter = iter_view(&v);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_func() {
        let mut vals = iter_view(&iter(&3, |v: &usize| 0..*v));
        assert_eq!(vals.next(), Some(0));
        assert_eq!(vals.next(), Some(1));
        assert_eq!(vals.next(), Some(2));
        assert_eq!(vals.next(), None);
    }
}
