use std::{any::TypeId, hash::Hash};

use crate::{components::Component, storage::{ComponentStorage, Storage}};

pub struct ComponentStorageBuilder<E> {
    storage: ComponentStorage<E>
}

impl<E: Eq + PartialEq + Hash + Copy + 'static> ComponentStorageBuilder<E> {
    pub fn new() -> Self {
        Self { storage: ComponentStorage::<E>::new() }
    }

    pub fn with<T: Component>(mut self) -> Self {
        self.storage.storages.insert(TypeId::of::<T>(), Box::new(Storage::<E, T>::default()));
        self
    }

    pub fn build(self) -> ComponentStorage<E> {
        self.storage
    }
}