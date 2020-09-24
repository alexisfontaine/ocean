mod kind;

#[cfg(feature = "story")]
mod story;


pub use self::kind::*;

use yew::prelude::*;

use crate::utils::ne_assign;


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
			<@{self.kind.tag()} class=(&self.class, self.kind.class())>
				{self.children.clone()}
			</@>
		}
	}
}
