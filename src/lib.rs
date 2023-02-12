//! Rust has `IntoIterator` trait, to create `Iterator` from a type. `IntoIterator` consumes the type,
//! many types provides `iter()` method to create `Iterator` from immutable reference without consuming the type.
//! But no trait for `iter()` method.
//!
//! In most cases, such trait is not needed, because `iter()` method returns `Iterator` which implements `IntoIterator` trait.
//! But consider the following case:
//!
//! ```rust
//!
//! trait Inspector<T> {
//!     fn inspect(&self, v: &T);
//! }
//!
//! ```
//!
//! `inspect()` method takes immutable reference of `T`, and `T` is not `Copy` type. If impl
//! `Inspector` for any object can convert into `Iterator`, we can write code like this:
//!
//! ```rust
//!
//! impl<T, I> Inspector<T> for I
//!     where
//!         I: IntoIterator<Item = T>,
//!         T: std::fmt::Debug,
//! {
//!     fn inspect(&self, v: &T) {
//!         for item in self {
//!             println!("{:?}", item);
//!         }
//!     }
//! }
//!
//! ```
//!
//! But `IntoIterator` trait consumes the type, compiler won't let it go.
//!
//! If we impl `Inspector` on slice:
//!
//! ```rust
//!
//! impl<T: std::fmt::Debug> Inspector<T> for [T] {
//!     fn inspect(&self, v: &T) {
//!         for item in self {
//!             println!("{:?}", item);
//!         }
//!     }
//! }
//!
//! ```
//!
//! Won't work, unless we change `Inspector` trait to allow unsized type:
//!
//! ```rust
//!
//! trait Inspector<T: ?Sized> {
//!     fn inspect(&self, v: &T);
//! }
//!
//! ```
//!
//! Unsized types are poison, cause much trouble to use, we don't want to use it.
//!
//! `Vec<T>` support convert to slice very easily, but many other types don't support it, such as `HashMap`,
//! `LinkedList`, slice version can not cover all cases.
//!
//! We can not pass `Iterator` to `inspect()` method, because `Iterator` is a mutable object, and `inspect()`
//! requires immutable reference, make it impossible to work.
//!
//! `iter_view` crate provides `IterView` trait, which is similar to `IntoIterator`, but it doesn't consume the type,
//!
//! ```rust
//!
//! pub trait IterView<'a> {
//!     type Item: 'a;
//!     type Iter: Iterator<Item = &'a Self::Item>;
//!     fn iter(&'a self) -> Self::Iter;
//! }
//! ```
//!
//! Use `IterView` trait, we can impl `Inspector` for any type which implements `IterView`:
//!
//! ```rust
//! use iter_view::IterView;
//!
//! impl<T, I> Inspector<T> for I
//!     where
//!         I: IterView<Item = T>,
//!         T: std::fmt::Debug,
//! {
//!     fn inspect(&self, v: &T) {
//!         for item in self.iter() {
//!             println!("{:?}", item);
//!         }
//!     }
//! }
//!
//! ```

use std::marker::PhantomData;
use std::slice;

/// Like IntoIterator, but not consume current object, return a readonly iterator.
pub trait IterView<'a> {
    type Item: 'a;
    type Iter: Iterator<Item = Self::Item>;
    fn iter(&'a self) -> Self::Iter;
}

impl<'a, T: 'a + IterView<'a> + ?Sized> IterView<'a> for &'a T {
    type Item = T::Item;
    type Iter = T::Iter;
    fn iter(&'a self) -> Self::Iter {
        (*self).iter()
    }
}

impl<'a, T: 'a, const N: usize> IterView<'a> for [T; N] {
    type Item = &'a T;
    type Iter = slice::Iter<'a, T>;
    fn iter(&'a self) -> Self::Iter {
        self[..].iter()
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

    fn iter_view<'a, T: IterView<'a> + ?Sized>(o: &'a T) -> T::Iter {
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

    #[test]
    fn iter_slice() {
        let v: &[u8] = &[1, 2, 3];
        let mut iter = iter_view(&v);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_array() {
        let v: [u8; 3] = [1, 2, 3];
        let mut iter = iter_view(&v);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }
}
