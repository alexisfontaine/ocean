#[cfg(feature = "story")]
mod story;


use yew::prelude::*;

use crate::utils::{ne_assign, style_file};


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct List {
	pub children: Children,

	#[prop_or_default]
	pub class: String,

	#[prop_or("section".into())]
	pub tag: String,
}


impl List {
	pub const STYLE: &'static str = style_file!("list", "./list.scss");
}


impl Component for List {
	type Message = ();

	type Properties = Self;


	fn create (properties: Self::Properties, _: ComponentLink<Self>) -> Self {
		properties
	}

	fn change (&mut self, properties: Self::Properties) -> ShouldRender {
		ne_assign(self, properties)
	}

	fn update (&mut self, _: Self::Message) -> ShouldRender {
		false
	}

	fn view (&self) -> Html {
		html! {
			<@{self.tag.clone()} class=(&self.class, Self::STYLE)>
				{self.children.clone()}
			</@>
		}
	}
}
