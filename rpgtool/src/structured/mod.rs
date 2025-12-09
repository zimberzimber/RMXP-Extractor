#![allow(
    dead_code,
    clippy::struct_field_names,
    clippy::struct_excessive_bools,
    clippy::cast_possible_truncation,
    unused_imports
)]

mod rgss_structs;
pub use rgss_structs::*;

mod shared;
pub use shared::*;

pub mod rmxp;
