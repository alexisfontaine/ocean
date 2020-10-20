#![recursion_limit="256"]
#![feature(cell_update, once_cell, option_result_contains, or_patterns)]


pub mod components;
pub mod utils;

#[cfg(feature = "router")]
pub mod router;

#[cfg(feature = "story")]
pub mod story;


pub use components::*;


utils::include_style_file!("main", "./main.scss");
