use std::{collections::HashSet, hash::Hash, marker::PhantomData};

use crate::{components::ComponentList, query, storage::ComponentStorage, Component};

pub trait Query<E> {
    fn query(storage: &ComponentStorage<E>) -> Vec<E>;
}

pub struct With<T: ComponentList>(PhantomData<T>);
pub struct Without<T: ComponentList>(PhantomData<T>);

impl<E: Eq + PartialEq + Hash + Copy + 'static, T: ComponentList> Query<E> for With<T> {
    fn query(storage: &ComponentStorage<E>) -> Vec<E> {
        let types = T::type_ids();
        let mut sets: Vec<HashSet<E>> = vec![];
        for type_id in types {
            if let Some(bucket) = storage.storages.get(&type_id) {
                let entities: HashSet<E> = HashSet::from_iter(bucket.get_entities());
                sets.push(entities);
            } else {
                return vec![]
            }
        }
        if sets.is_empty() {
            return vec![]
        }
        let mut iter = sets.into_iter();
        let first = iter.next().unwrap(); // Safe to unwrap because vec is non empty
        let intersection = iter.fold(first, |acc, next| {
            acc.intersection(&next).copied().collect()
        });
        intersection.into_iter().collect()
    }
}

impl<E: Eq + PartialEq + Hash + Copy + 'static, T: ComponentList> Query<E> for Without<T> {
    fn query(storage: &ComponentStorage<E>) -> Vec<E> {
        let all: HashSet<E> = HashSet::from_iter(storage.all_entities());
        let mut exclude_set: HashSet<E> = HashSet::new();
        for type_id in T::type_ids() {
            if let Some(bucket) = storage.storages.get(&type_id) {
                exclude_set.extend(bucket.get_entities());
            }
        }
        all.difference(&exclude_set).into_iter().copied().collect()
    }
}

impl<E: Eq + PartialEq + Hash + Copy + 'static, T: ComponentList, U: ComponentList> Query<E> for (With<T>, Without<U>) {
    fn query(storage: &ComponentStorage<E>) -> Vec<E> {
        let with: HashSet<E> = HashSet::from_iter(<With<T>>::query(storage));
        let without: HashSet<E> = HashSet::from_iter(<Without<U>>::query(storage));
        with.intersection(&without).copied().collect::<Vec<E>>() 
    }
}

impl<E: Eq + PartialEq + Hash + Copy + 'static, T: ComponentList, U: ComponentList> Query<E> for (Without<U>, With<T>) {
    fn query(storage: &ComponentStorage<E>) -> Vec<E> {
        let with: HashSet<E> = HashSet::from_iter(<With<T>>::query(storage));
        let without: HashSet<E> = HashSet::from_iter(<Without<U>>::query(storage));
        with.intersection(&without).copied().collect::<Vec<E>>() 
    }
}