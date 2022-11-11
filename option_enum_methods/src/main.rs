use std::mem;
struct SomeErr;

#[derive(Debug)]
pub enum Option<T> {
    Some(T), // Some value of type T
    None,    // No value
}

impl<T> Option<T> {
    pub const fn is_some(&self) -> bool {
        /*
        matches! checks whether the given expression matches any of the given patterns

        let foo = 'f';
        assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

        let bar = Some(4);
        assert!(matches!(bar, Some(x) if x > 2));
        */
        matches!(*self, Option::Some(_))
    }

    pub fn is_some_and(&self, f: impl FnOnce(&T) -> bool) -> bool {
        matches!(self, Option::Some(x) if f(&x))
    }

    pub const fn is_none(&self) -> bool {
        !self.is_some()
    }

    pub const fn as_ref(&self) -> Option<&T> {
        match *self {
            Option::Some(ref x) => Option::Some(x),
            Option::None => Option::None,
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        match *self {
            Option::Some(ref mut x) => Option::Some(x),
            Option::None => Option::None,
        }
    }

    pub fn expect(self, msg: &str) -> T {
        match self {
            Option::Some(val) => val,
            Option::None => panic!("{}", msg),
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            Option::Some(val) => val,
            Option::None => panic!(""),
        }
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Option::Some(val) => val,
            Option::None => default,
        }
    }

    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Option::Some(val) => val,
            Option::None => f(),
        }
    }

    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Option::Some(val) => val,
            Option::None => Default::default(),
        }
    }

    pub unsafe fn unwrap_unchecked(self) -> T {
        match self {
            Option::Some(val) => val,
            Option::None => unsafe { hint::unreachable_unchecked() },
        }
    }

    pub fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Option::Some(x) => Option::Some(f(x)),
            Option::None => Option::None,
        }
    }

    pub fn map_or<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Option::Some(val) => f(val),
            Option::None => default,
        }
    }

    pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
    {
        match self {
            Option::Some(x) => f(x),
            Option::None => default(),
        }
    }

    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Option::Some(val) => Result::Ok(val),
            Option::None => Result::Err(err),
        }
    }

    pub fn ok_or_else<E, F>(self, err: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Option::Some(val) => Result::Ok(val),
            Option::None => Result::Err(err()),
        }
    }

    pub fn and<U>(self, optb: Option<U>) -> Option<U> {
        match self {
            Option::Some(_) => optb,
            Option::None => Option::None,
        }
    }

    pub fn and_then<F, U>(self, optb: F) -> Option<U>
    where
        F: FnOnce(T) -> Option<U>,
    {
        match self {
            Option::Some(x) => optb(x),
            Option::None => Option::None,
        }
    }

    pub fn filter<P>(self, predicate: P) -> Option<T>
    where
        P: FnOnce(&T) -> bool,
    {
        if let Option::Some(x) = self {
            if predicate(&x) {
                return Option::Some(x);
            }
        }
        Option::None
    }

    pub fn or(self, optb: Option<T>) -> Option<T> {
        match self {
            Option::Some(x) => Option::Some(x),
            Option::None => optb,
        }
    }

    pub fn or_else<F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> Option<T>,
    {
        match self {
            Option::Some(x) => Option::Some(x),
            Option::None => f(),
        }
    }

    pub fn xor(self, optb: Option<T>) -> Option<T> {
        match (self, optb) {
            (Option::Some(a), Option::None) => Option::Some(a),
            (Option::None, Option::Some(b)) => Option::Some(b),
            _ => Option::None,
        }
    }

    pub fn insert(&mut self, value: T) -> &mut T {
        *self = Option::Some(value);

        unsafe { self.as_mut().unwrap_unchecked() }
    }

    pub fn get_or_insert(&mut self, value: T) -> &mut T {
        if let Option::None = *self {
            *self = Option::Some(value);
        }

        unsafe { self.as_mut().unwrap_unchecked() }
    }

    pub fn take(&mut self) -> Option<T> {
        mem::replace(self, Option::None)
    }

    pub fn replace(&mut self, value: T) -> Option<T> {
        mem::replace(self, Option::Some(value))
    }

    pub fn contains<U>(&self, x: &U) -> bool
    where
        U: PartialEq<T>,
    {
        if let Option::Some(val) = self {
            return x.eq(val);
        }

        false
    }

    pub fn zip<U>(self, other: Option<U>) -> Option<(T, U)> {
        match (self, other) {
            (Option::Some(x), Option::Some(y)) => Option::Some((x, y)),
            _ => Option::None,
        }
    }

    pub fn zip_with<U, F, R>(self, other: Option<U>, f: F) -> Option<R>
    where
        F: FnOnce(T, U) -> R,
    {
        match (self, other) {
            (Option::Some(x), Option::Some(y)) => Option::Some(f(x, y)),
            _ => Option::None,
        }
    }
}

impl<T, U> Option<(T, U)> {
    pub fn unzip(self) -> (Option<T>, Option<U>) {
        match self {
            Option::Some((a, b)) => (Option::Some(a), Option::Some(b)),
            _ => (Option::None, Option::None),
        }
    }
}

impl<T> Option<&T> {
    pub fn copied(self) -> Option<T>
    where
        T: Copy,
    {
        match self {
            Option::Some(&x) => Option::Some(x),
            Option::None => Option::None,
        }
    }

    pub fn cloned(self) -> Option<T>
    where
        T: Clone,
    {
        match self {
            Option::Some(x) => Option::Some(x.clone()),
            Option::None => Option::None,
        }
    }
}

impl<T> Option<&mut T> {
    pub fn copied(&mut self) -> Option<T>
    where
        T: Copy,
    {
        match self {
            Option::Some(&mut x) => Option::Some(x),
            Option::None => Option::None,
        }
    }

    pub fn cloned(&mut self) -> Option<T>
    where
        T: Clone,
    {
        match self {
            Option::Some(x) => Option::Some(x.clone()),
            Option::None => Option::None,
        }
    }
}

impl<T, E> Option<Result<T, E>> {
    pub fn transpose(self) -> Result<Option<T>, E> {
        match self {
            Option::Some(Ok(x)) => Ok(Option::Some(x)),
            Option::Some(Err(e)) => Err(e),
            Option::None => Ok(Option::None),
        }
    }
}

impl<T> Option<Option<T>> {
    pub fn flatten(self) -> Option<T> {
        match self {
            Option::Some(inner) => inner,
            Option::None => Option::None,
        }
    }
}

fn main() {}
