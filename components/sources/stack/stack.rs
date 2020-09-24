#[cfg(feature = "story")]
mod story;


use yew::prelude::*;

use crate::utils::ne_assign;


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Stack {
	pub children: Children,

	#[prop_or_default]
	pub class: String,

	#[prop_or_else(|| "section".into())]
	pub tag: String,
}


impl Component for Stack {
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
			<@{self.tag.clone()} class=(&self.class, "stack")>
				{self.children.clone()}
			</@>
		}
	}
}
