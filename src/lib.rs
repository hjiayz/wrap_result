//! # Examples
//!
//! ```
//! use wrap_result::*;
//! let x = 0;
//! assert_eq!(x.wrap_ok(), Result::<u32,u32>::Ok(x));
//!
//! assert_eq!(x.wrap_err(), Result::<u32,u32>::Err(x));
//!
//! assert_eq!(x.wrap_some(), Some(x));
//! ```

pub trait WrapOk<E>: Sized {
    fn wrap_ok(self) -> Result<Self, E>;
}

impl<T, E> WrapOk<E> for T {
    #[inline]
    fn wrap_ok(self) -> Result<Self, E> {
        Ok(self)
    }
}

pub trait WrapErr<O>: Sized {
    fn wrap_err(self) -> Result<O, Self>;
}

impl<T, O> WrapErr<O> for T {
    #[inline]
    fn wrap_err(self) -> Result<O, Self> {
        Err(self)
    }
}

pub trait WrapSome: Sized {
    fn wrap_some(self) -> Option<Self>;
}

impl<T> WrapSome for T {
    #[inline]
    fn wrap_some(self) -> Option<Self> {
        Some(self)
    }
}
