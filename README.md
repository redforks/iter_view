# `iter_view` crate for `rust`

Rust has `IntoIterator` trait, to create `Iterator` from a type. `IntoIterator` consumes the type, 
many types provides `iter()` method to create `Iterator` from immutable reference without consuming the type.
But no trait for `iter()` method.

In most cases, such trait is not needed, because `iter()` method returns `Iterator` which implements `IntoIterator` trait.
But consider the following case:

```rust

trait Inspector<T> {
    fn inspect(&self, v: &T);
}

```

`inspect()` method takes immutable reference of `T`, and `T` is not `Copy` type. If impl
`Inspector` for any object can convert into `Iterator`, we can write code like this:

```rust

impl<T, I> Inspector<T> for I
where
    I: IntoIterator<Item = T>,
    T: std::fmt::Debug, 
{
    fn inspect(&self, v: &T) {
        for item in self {
            println!("{:?}", item);
        } 
    }
}

```

But `IntoIterator` trait consumes the type, compiler won't let it go.

If we impl `Inspector` on slice:

```rust

impl<T: std::fmt::Debug> Inspector<T> for [T] {
    fn inspect(&self, v: &T) {
        for item in self {
            println!("{:?}", item);
        } 
    }
}

```

Won't work, unless we change `Inspector` trait to allow unsized type:

```rust

trait Inspector<T: ?Sized> {
    fn inspect(&self, v: &T);
}

```

Unsized types are poison, cause much trouble to use, we don't want to use it.

`Vec<T>` support convert to slice very easily, but many other types don't support it, such as `HashMap`,
`LinkedList`, slice version can not cover all cases.

We can not pass `Iterator` to `inspect()` method, because `Iterator` is a mutable object, and `inspect()`
requires immutable reference, make it impossible to work.

`iter_view` crate provides `IterView` trait, which is similar to `IntoIterator`, but it doesn't consume the type,

```rust

pub trait IterView<'a> {
    type Item: 'a;
    type Iter: Iterator<Item = &'a Self::Item>;
    fn iter(&'a self) -> Self::Iter;
}
```

Use `IterView` trait, we can impl `Inspector` for any type which implements `IterView`:

```rust

impl<T, I> Inspector<T> for I
where
    I: IterView<Item = T>,
    T: std::fmt::Debug, 
{
    fn inspect(&self, v: &T) {
        for item in self.iter() {
            println!("{:?}", item);
        } 
    }
}

```

