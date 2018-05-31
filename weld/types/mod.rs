//! Types in the Weld IR.
//!
//! Weld contains a number of builtin types, ranging from scalars (e.g., floating-point and integer
//! values such as `f32` and `i64`) to collection types (e.g., `vec[i32]` and structs like
//! `{i32,i64,vec[f32]}`. Types can be nested arbitrarily.
//!
//! ## Builders
//!
//! The main interesting types in Weld are the builders, which represent mutable values that
//! support multi-threaded writes.
//!
//! TODO add some description on what each of the builders do.

pub use self::types::*;

mod types;
pub mod infer;
