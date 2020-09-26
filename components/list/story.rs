use crate::story::*;
use super::*;


impl Story for List {
	const TITLE: &'static str = "List";


	fn gallery () -> Vec<Example> {
		vec! [
			Example::from(html! {
				<List>
					<span>{"Item 0"}</span>
					<span class="active">{"Item 1"}</span>
					<span>{"Item 2"}</span>
				</List>
			}),
		]
	}
}
