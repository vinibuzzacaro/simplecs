//! # simplecs
//!
//! [![Crates.io](https://img.shields.io/crates/v/simplecs.svg)](https://crates.io/crates/simplecs)
//! [![Documentation](https://docs.rs/simplecs/badge.svg)](https://docs.rs/simplecs)
//! [![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/simplecs)](https://opensource.org/licenses/MIT)
//!
//! **`simplecs`** is a lightweight, zero-dependency Entity-Component System (ECS) for Rust.  
//! It emphasizes **simplicity**, **type-safety**, and **ergonomic querying**.
//!
//! ---
//!
//! ## ✨ Features
//!
//! - ✅ Simple API — no macro-heavy boilerplate
//! - 🧠 Type-based querying with `With<T>` / `Without<T>`
//! - 🧩 Dynamic component registration via `ComponentStorageBuilder`
//! - 🔄 Uses `TypeId` + `Any` internally for type-erased buckets
//! - 🦀 Pure safe Rust — great for learning ECS
//!
//! ---
//!
//! ## 🚀 Example
//!
//! ```rust
//! use simplecs::*;
//!W
//! struct Position(f32, f32);
//! impl Component for Position {}
//!
//! struct Velocity(f32, f32);
//! impl Component for Velocity {}
//!
//! // For now, you should manually inform 
//! // how many combinations you can use in
//! // your queries
//! component_list!(A, B, C, D, E);
//!
//! fn main() {
//!     let mut storage = ComponentStorageBuilder::<u32>::new()
//!         .with::<Position>()
//!         .with::<Velocity>()
//!         .build();
//!
//!     storage.add_component(1, Position(1.0, 2.0));
//!     storage.add_component(1, Velocity(0.1, 0.1));
//!     storage.add_component(2, Position(5.0, 5.0));
//!
//!     let moving = storage.query::<With<(Position, Velocity)>>();
//!     assert_eq!(moving, vec![1]);
//!
//!     let static_entities = storage.query::<(With<Position>, Without<Velocity>)>();
//!     assert_eq!(static_entities, vec![2]);
//! }
//! ```
//!
//! ---
//!
//! ## 📐 Query API
//!
//! `simplecs` uses type-safe static dispatch to query entities:
//!
//! - `With<T>` – selects entities with components `T`
//! - `Without<T>` – selects entities that lack `T`
//! - Combinations like `(With<A>, Without<B>)` work too
//!
//! Define component groups with a macro:
//!
//! ```rust
//! component_list!(A, B, C, D, E, F);
//! ```
//!
//! Then use in queries:
//!
//! ```rust
//! let ids = storage.query::<(With<(Position, Velocity)>, Without<Health>)>();
//! ```
//!
//! ---
//!
//! ## 🧱 Architecture
//!
//! - Each component type has a separate `Storage<E, T>`
//! - All storages are type-erased into `dyn ComponentBucket<E>`
//! - ECS queries are resolved using sets and intersections
//! - Entities can be any `Copy + Eq + Hash` type (e.g. `u32`)
//!
//! ---
//!
//! ## 🛠 License
//!
//! Licensed under either of:
//!
//! - MIT license <https://opensource.org/licenses/MIT>
//! - Apache License, Version 2.0 <https://www.apache.org/licenses/LICENSE-2.0>
//!
//! ---
//!
//! ## 🦀 Created by Buzzac
//!
//! For questions, ideas, or contributions, feel free to open an issue or PR.
//!
//! Want to regenerate this README from your docs?
//!
//! ```sh
//! cargo install cargo-readme
//! cargo readme > README.md
//! ```


mod components;
mod storage;
mod query;
mod builder;

pub use crate::builder::ComponentStorageBuilder;
pub use crate::components::{Component, ComponentBucket, ComponentList};
pub use crate::query::{Query, With, Without};
pub use crate::storage::{ComponentStorage, Storage};
pub use derive_lib::Component;