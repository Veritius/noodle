use core::{any::Any, ops::Deref};

/// A reference to a value.
#[derive(Clone, Copy)]
pub struct ValueRef<'a> {
    value: &'a dyn Any,
}

impl<'a> ValueRef<'a> {
    /// Returns a new [`ValueRef`] for `value`.
    #[inline]
    pub fn new<T: Any>(value: &'a T) -> ValueRef<'a> {
        Self { value }
    }

    /// Returns the inner `dyn Any` borrow with the same lifetime as `self`.
    #[inline]
    pub fn inner_any(&'a self) -> &'a dyn Any {
        self.value
    }

    /// Tries to cast the inner borrow to `T`.
    #[inline]
    pub fn downcast<T: Any>(&'a self) -> Result<&'a T, TypeCastError> {
        self.value.downcast_ref().ok_or(TypeCastError)
    }
}

impl Deref for ValueRef<'_> {
    type Target = dyn Any;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

/// A type that can be put in a [`Value`].
pub trait Valuelike: Any + Send + Sync + 'static {}

impl<T> Valuelike for T where T: Any + Send + Sync + 'static {}

/// Returned when attempting to cast a [`Value`] to a concrete type fails.
#[derive(Debug, Clone, Copy)]
pub struct TypeCastError;