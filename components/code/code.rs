#[cfg(feature = "story")]
mod story;

pub use ocean_macros::{highlight, highlight_file};

use web_sys::window;
use yew::prelude::*;

use crate::utils::{ne_assign, style_file};


macro_rules! style {
	() => {style_file!("code", "components/code/code.scss")};
}


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Code {
	#[prop_or_default]
	pub class: String,

	pub content: &'static str,

	#[prop_or_default]
	pub inline: bool,
}


impl Code {
	pub const fn class (&self) -> &'static str {
		if self.inline
			{ concat!(style!(), "--inline") } else
			{ style!() }
	}
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

		class.push_str(self.class());
		container.set_class_name(&class);
		container.set_inner_html(&self.content);
		Html::VRef(container.into())
	}
}
