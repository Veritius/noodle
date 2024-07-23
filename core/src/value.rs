use core::any::{Any, TypeId};

/// A dynamically typed value used in a graph.
#[derive(Debug, Clone, Copy)]
pub struct Value {
    v_type: ValueTypeId,
}

/// Dynamic representation of a [`Value`]'s type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValueTypeId(ValueTypeInner);

impl ValueTypeId {
    /// Returns a [`ValueTypeId`] for the Rust type `T`.
    // TODO: Make this function const when TypeId::of is made const
    pub fn from_any<T: Any>() -> Self {
        Self(ValueTypeInner::CompileType(TypeId::of::<T>()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ValueTypeInner {
    CompileType(TypeId),
}