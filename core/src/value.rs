use core::any::TypeId;

pub trait Valuelike: 'static {
    fn type_id(&self) -> ValueType;
}

impl<T: Valuelike> Valuelike for [T] {
    fn type_id(&self) -> ValueType {
        ValueType::from_typeid(TypeId::of::<[T]>())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValueType(ValueTypeInner);

impl ValueType {
    pub fn from_typeid(id: TypeId) -> Self {
        ValueType(ValueTypeInner::Static(id))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ValueTypeInner {
    Static(TypeId),
}

macro_rules! primitive {
    ($type:ty) => {
        impl Valuelike for $type {
            fn type_id(&self) -> ValueType {
                ValueType::from_typeid(TypeId::of::<$type>())
            }
        }
    };
}

primitive!(bool);
primitive!(char);
primitive!(str);
primitive!(f32);
primitive!(f64);
primitive!(u8);
primitive!(u16);
primitive!(u32);
primitive!(u64);
primitive!(u128);
primitive!(i8);
primitive!(i16);
primitive!(i32);
primitive!(i64);
primitive!(i128);