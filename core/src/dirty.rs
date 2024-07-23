use core::ops::{Deref, DerefMut};

/// A value that can be made 'dirty'.
pub trait Dirty {
    /// Returns `true` if the value is dirtied.
    fn is_dirty(&self) -> bool;

    /// Sets the value as undirtied.
    fn clear_dirty(&mut self);
}

/// A wrapper around any `T` that implements [`Dirty`].
/// 
/// `Dirtiable` will be set as dirty whenever mutably dereferenced.
pub struct Dirtiable<T> {
    value: T,
    dirty: bool,
}

impl<T> Dirty for Dirtiable<T> {
    #[inline]
    fn is_dirty(&self) -> bool {
        self.dirty
    }

    #[inline]
    fn clear_dirty(&mut self) {
        self.dirty = false;
    }
}

impl<T> Dirtiable<T> {
    /// Creates a new `Dirtiable` from `T`.
    pub fn new(value: T) -> Self {
        Self {
            value,
            dirty: false,
        }
    }

    /// Returns the inner value and whether it was dirtied.
    pub fn into_inner(self) -> (T, bool) {
        (self.value, self.dirty)
    }

    /// Returns a mutable reference to the inner value,
    /// without flagging the value as being dirty.
    pub unsafe fn bypass(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> From<T> for Dirtiable<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> Deref for Dirtiable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Dirtiable<T> {
    // This is the closest we can get to change detection
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.dirty = true;
        &mut self.value
    }
}

impl<T> AsRef<T> for Dirtiable<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        Deref::deref(self)
    }
}

impl<T> AsMut<T> for Dirtiable<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        DerefMut::deref_mut(self)
    }
}

impl<T> AsRef<Dirtiable<T>> for Dirtiable<T> {
    fn as_ref(&self) -> &Dirtiable<T> {
        self
    }
}

impl<T> AsMut<Dirtiable<T>> for Dirtiable<T> {
    fn as_mut(&mut self) -> &mut Dirtiable<T> {
        self
    }
}