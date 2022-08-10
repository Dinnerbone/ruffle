#![deny(clippy::missing_inline_in_public_items)]

#[macro_use]
extern crate num_derive;

pub mod backend;
pub mod binary_data;
pub mod bounding_box;
pub mod color_transform;
pub mod display_object;
pub mod ecma_conversions;
pub mod either;
pub mod events;
pub mod loader;
pub mod matrix;
pub mod numbers;
pub mod shape_utils;
pub mod string;
pub mod tag_utils;
pub mod transform;
pub mod vminterface;
