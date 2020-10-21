mod kind;

#[cfg(feature = "story")]
mod story;


pub use self::kind::*;

use yew::prelude::*;

use crate::utils::ne_assign;


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Anchor {
	pub children: Children,

	#[prop_or_default]
	pub class: String,

	#[prop_or_default]
	pub disabled: bool,

	#[prop_or(Anchor::Standalone)]
	pub kind: Kind,

	#[prop_or_default]
	pub loading: bool,

	#[prop_or_default]
	pub open_in_new_context: bool,

	pub to: String,
}


#[allow(non_upper_case_globals)]
impl Anchor {
	pub const Button: Kind = Kind::Button(ButtonModifier::Default);

	pub const ButtonDanger: Kind = Kind::Button(ButtonModifier::Danger);

	pub const ButtonPrimary: Kind = Kind::Button(ButtonModifier::Primary);

	pub const ButtonSecondary: Kind = Kind::Button(ButtonModifier::Secondary);

	pub const Inline: Kind = Kind::Anchor(AnchorModifier::Inline);

	pub const Standalone: Kind = Kind::Anchor(AnchorModifier::Standalone);
}

impl Component for Anchor {
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
		render(&self.to, self.kind, self.class.clone(), None, self.children.clone(), false, self.disabled, self.loading, self.open_in_new_context)
	}
}


pub fn render (to: &str, kind: Kind, mut class: String, handle_click: Option<&Callback<MouseEvent>>, children: Children, is_active: bool, is_disabled: bool, is_loading: bool, open_in_new_context: bool) -> Html {
	if !class.is_empty()
		{ class.push(' '); }

	class.push_str(kind.class());

	if is_active
		{ class.push_str(" active"); }

	if is_disabled
		{ class.push_str(" disabled"); }

	if is_loading
		{ class.push_str(" loading"); }

	if is_loading || is_disabled {
		html!(<a class=class>{children}</a>)
	} else if let Some(handle_click) = handle_click {
		if open_in_new_context
			{ html!(<a class=class href=to onclick=handle_click rel="noopener" target="_blank">{children}</a>) } else
			{ html!(<a class=class href=to onclick=handle_click>{children}</a>) }
	} else if open_in_new_context {
		html!(<a class=class href=to rel="noopener" target="_blank">{children}</a>)
	} else {
		html!(<a class=class href=to>{children}</a>)
	}
}
