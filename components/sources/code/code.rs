#[cfg(feature = "story")]
mod story;

pub use ocean_macro::{highlight, highlight_file};

use web_sys::window;
use yew::prelude::*;

use crate::utils::ne_assign;


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Code {
	#[prop_or_default]
	pub class: String,

	pub content: &'static str,

	#[prop_or_default]
	pub inline: bool,
}


impl Component for Code {
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
		let mut class = self.class.clone();
		let container = window().unwrap().document().unwrap().create_element(if self.inline { "code" } else { "pre" }).unwrap();

		if class.is_empty()
			{ class.push_str(if self.inline { "code--inline" } else { "code" }) }

		container.set_class_name(&class);
		container.set_inner_html(&self.content);
		Html::VRef(container.into())
	}
}
