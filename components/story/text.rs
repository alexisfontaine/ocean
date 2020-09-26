use yew::prelude::*;


pub fn paragraph_1 () -> Html {
	html! {
		<p>{TEXT_1}</p>
	}
}

pub fn paragraph_2 () -> Html {
	html! {
		<p>{TEXT_2}</p>
	}
}


pub const TEXT_1: &str = "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";

pub const TEXT_2: &str = "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
