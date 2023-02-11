use std::slice;

/// Like IntoIterator, but not consume current object, return a readonly iterator.
pub trait IterView<'a> {
    type Item: 'a;
    type Iter: Iterator<Item = &'a Self::Item>;
    fn iter(&'a self) -> Self::Iter;
}

impl<'a, T: 'a> IterView<'a> for Vec<T> {
    type Item = T;
    type Iter = slice::Iter<'a, Self::Item>;
    fn iter(&'a self) -> Self::Iter {
        self[..].iter()
    }
}

impl<'a, T: 'a> IterView<'a> for [T] {
    type Item = T;
    type Iter = slice::Iter<'a, Self::Item>;
    fn iter(&'a self) -> Self::Iter {
        self.iter()
    }
}

impl<'a, T: 'a> IterView<'a> for Option<T> {
    type Item = T;
    type Iter = std::option::Iter<'a, T>;
    fn iter(&'a self) -> Self::Iter {
        self.iter()
    }
}

// impl IterView for Result
impl<'a, T: 'a, E: 'a> IterView<'a> for Result<T, E> {
    type Item = T;
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
}
