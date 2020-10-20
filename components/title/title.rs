#[cfg(feature = "story")]
mod story;


use yew::prelude::*;

use crate::utils::style_file;
use crate::utils::ne_assign;


macro_rules! style {
	() => {style_file!("title", "components/title/title.scss")}
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
	Headline,
	Main,
	Section,
	SubSection,
}


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Title {
	pub children: Children,

	#[prop_or_default]
	pub class: String,

	#[prop_or(Kind::Main)]
	pub kind: Kind,
}


#[allow(non_upper_case_globals)]
impl Title {
	pub const Headline: Kind = Kind::Headline;

	pub const Main: Kind = Kind::Main;

	pub const Section: Kind = Kind::Section;

	pub const SubSection: Kind = Kind::SubSection;


	pub const fn class (&self) -> &'static str {
		match self.kind {
			Kind::Headline => concat!(style!(), "--headline"),
			Kind::Main => style!(),
			Kind::Section => concat!(style!(), "--section"),
			Kind::SubSection => concat!(style!(), "--sub-section"),
		}
	}

	pub const fn tag (&self) -> &'static str {
		match self.kind {
			Kind::Headline => "h1",
			Kind::Main => "h1",
			Kind::Section => "h2",
			Kind::SubSection => "h3",
		}
	}
}


impl Component for Title {
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
			<@{self.tag()} class=(&self.class, self.class())>
				{self.children.clone()}
			</@>
		}
	}
}
