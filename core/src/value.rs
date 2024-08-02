use core::any::Any;
use alloc::sync::Arc;

/// A type-erased reference-counted value used in node graph calculations.
#[derive(Clone)]
#[repr(transparent)]
pub struct Value(ValueInner);

impl Value {
    /// Attempts to cast the [`Value`] to `T`. Fails if the value is not of type `T`.
    #[inline(always)]
    pub fn downcast<'a, T: Valuelike>(&'a self) -> Result<&'a T, TypeCastError> {
        self.0.downcast::<T>()
    }

    /// Creates a new [`Value`] holding `value`.
    #[inline(always)]
    pub fn new<T: Valuelike>(value: T) -> Self {
        Self(ValueInner::new(value))
    }
}

impl AsRef<dyn Valuelike> for Value {
    #[inline(always)]
    fn as_ref(&self) -> &dyn Valuelike {
        self.0.value.as_ref()
    }
}

/// A type that can be put in a [`Value`].
pub trait Valuelike: Any + Send + Sync + 'static {}

impl<T> Valuelike for T where T: Any + Send + Sync + 'static {}

#[derive(Clone)]
struct ValueInner {
    value: Arc<dyn Valuelike>,
}

impl ValueInner {
    fn downcast<'a, T: Valuelike>(&'a self) -> Result<&'a T, TypeCastError> {
        <dyn Any>::downcast_ref(&self.value).ok_or(TypeCastError)
    }

    fn new<T: Valuelike>(value: T) -> Self {
        Self {
            value: Arc::new(value),
        }
    }
}

impl AsRef<dyn Valuelike> for ValueInner {
    #[inline(always)]
    fn as_ref(&self) -> &dyn Valuelike {
        self.value.as_ref()
    }
}

/// Returned when attempting to cast a [`Value`] to a concrete type fails.
#[derive(Debug, Clone, Copy)]
pub struct TypeCastError;