#![feature(once_cell)]
#![recursion_limit="256"]


mod application;
mod pages;
mod route;


use wasm_bindgen::prelude::*;
use yew::start_app;

use application::Application;


#[wasm_bindgen(start)]
pub fn start () {
	start_app::<Application>();
}


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
mod allocator {
	use wee_alloc::WeeAlloc;


	#[global_allocator]
	static ALLOCATOR: WeeAlloc = WeeAlloc::INIT;
}
