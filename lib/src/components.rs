use std::any::{Any, TypeId};

pub trait Component: 'static {}

pub trait ComponentBucket<E> {
    fn get_entities(&self) -> Box<[E]>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait ComponentList: 'static {
    fn type_ids() -> Box<[TypeId]>;
}

#[macro_export]
macro_rules! component_list {
    ($first:ident) => {
        impl<$first: Component> crate::ComponentList for $first {
            fn type_ids() -> Box<[TypeId]> {
                vec![TypeId::of::<$first>()].into_boxed_slice()
            }
        }
    };

    ($first:ident, $($rest:ident),*) => {
        impl<$first: Component, $($rest: Component),*> crate::ComponentList for ($first, $($rest),*) {
            fn type_ids() -> Box<[TypeId]> {
                vec![TypeId::of::<$first>(), $(TypeId::of::<$rest>()),*].into_boxed_slice()
            }
        }
        component_list!($($rest),*);
    };
}