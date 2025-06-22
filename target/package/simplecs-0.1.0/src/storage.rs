use std::{any::{Any, TypeId}, collections::{HashMap, HashSet}, hash::Hash};

use crate::{components::{Component, ComponentBucket}, query::Query};

pub struct Storage<E: 'static + PartialEq + Eq + Hash, T: Component> {
    entities: Vec<E>,
    components: Vec<T>
}

pub struct ComponentStorage<E> {
    pub storages: HashMap<TypeId, Box<dyn ComponentBucket<E>>>
}

impl<E: PartialEq + Eq + Hash + Copy, T: Component> ComponentBucket<E> for Storage<E, T> {
    fn get_entities(&self) -> Box<[E]> {
        self.entities.iter().copied().collect::<Vec<E>>().into_boxed_slice()
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<E: PartialEq + Eq + Hash, T: Component> Default for Storage<E, T> {
    fn default() -> Self {
        Self { 
            entities: Default::default(), 
            components: Default::default() 
        }
    }
}
impl<E: PartialEq + Eq + Hash, T: Component> Storage<E, T> {
    pub fn index_of(&self, entity: &E) -> Option<usize> {
        self.entities.iter().position(|e| e == entity)
    }

    pub fn add_component(&mut self, entity: E, component: T) {
        self.entities.push(entity);
        self.components.push(component);
    }

    pub fn remove_component(&mut self, entity: &E) {
        if let Some(idx) = self.index_of(entity) {
            self.entities.swap_remove(idx);
            self.components.swap_remove(idx);
        }
    }

    pub fn get_component(&self, entity: &E) -> Option<&T> {
        self.index_of(entity).and_then(|idx| self.components.get(idx))
    }

    pub fn get_component_mut(&mut self, entity: &E) -> Option<&mut T> {
        self.index_of(entity).and_then(|idx| self.components.get_mut(idx))
    }
}

impl<E: PartialEq + 'static + Hash + Eq + Copy> ComponentStorage<E> {
    pub fn new() -> Self {
        Self { storages: HashMap::new() }
    }

    pub fn query<Q: Query<E>>(&self) -> Vec<E> {
        Q::query(self)
    }

    pub fn all_entities(&self) -> Box<[E]> {
        let mut set = HashSet::<E>::new();
        for storage in self.storages.values() {
            set.extend(storage.get_entities().into_iter());
        }
        set.iter().copied().collect::<Vec<E>>().into_boxed_slice()
    }

    fn get_storage<T: Component>(&self) -> Option<&Storage<E, T>> {
        self.storages
            .get(&TypeId::of::<T>())
            .and_then(|b| b.as_any().downcast_ref::<Storage<E, T>>())
    }

    fn get_storage_mut<T: Component>(&mut self) -> Option<&mut Storage<E, T>> {
        self.storages
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(Storage::<E, T>::default()))
            .as_any_mut()
            .downcast_mut::<Storage<E, T>>()
    }

    pub fn add_component<T: Component>(&mut self, entity: E, component: T) {
        if let Some(storage)= self.get_storage_mut::<T>() {
            storage.add_component(entity, component);
        }
    }

    pub fn remove_component<T: Component>(&mut self, entity: &E) {
        if let Some(storage) = self.get_storage_mut::<T>() {
            storage.remove_component(entity);
        }
    }

    pub fn get_component<T: Component>(&self, entity: &E) -> Option<&T> {
        self.get_storage::<T>().and_then(|s| s.get_component(entity))
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: &E) -> Option<&mut T> {
        self.get_storage_mut::<T>().and_then(|s| s.get_component_mut(entity))
    }
}