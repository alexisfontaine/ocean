#[cfg(feature = "story")]
mod story;


use yew::prelude::*;

use crate::utils::ne_assign;
use super::anchor::{render, Kind, AnchorModifier, ButtonModifier};


#[derive(Clone, Debug, PartialEq)]
pub enum Effect {
	None,
	Reset,
	Submit
}


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Action {
	pub children: Children,

	#[prop_or_default]
	pub class: String,

	#[prop_or_default]
	pub disabled: bool,

	#[prop_or(Action::None)]
	pub effect: Effect,

	#[prop_or(Action::Button)]
	pub kind: Kind,

	#[prop_or_default]
	pub loading: bool,

	#[prop_or(DEFAULT_HANDLER.with(|handler| (*handler).clone()))]
	pub onclick: Callback<MouseEvent>,
}


#[allow(non_upper_case_globals)]
impl Action {
	pub const AnchorInline: Kind = Kind::Anchor(AnchorModifier::Inline);

	pub const Anchor: Kind = Kind::Anchor(AnchorModifier::Standalone);

	pub const Button: Kind = Kind::Button(ButtonModifier::Default);

	pub const ButtonDanger: Kind = Kind::Button(ButtonModifier::Danger);

	pub const ButtonPrimary: Kind = Kind::Button(ButtonModifier::Primary);

	pub const ButtonSecondary: Kind = Kind::Button(ButtonModifier::Secondary);

	pub const None: Effect = Effect::None;

	pub const Reset: Effect = Effect::Reset;

	pub const Submit: Effect = Effect::Submit;
}

impl Effect {
	const fn value (&self) -> &'static str {
		match self {
			Self::None => "button",
			Self::Reset => "reset",
			Self::Submit => "submit"
		}
	}
}

impl Component for Action {
	type Message = ();

	type Properties = Self;


	fn create (properties: Self::Properties, _: ComponentLink<Self>) -> Self {
		debug_assert!(!matches!(properties, Self::Properties { effect: Effect::Reset | Effect::Submit, kind: Kind::Anchor(_), .. }), "Effects only works on anchors.");
		properties
	}

	fn change (&mut self, properties: Self::Properties) -> ShouldRender {
		ne_assign(self, properties)
	}

	fn update (&mut self, _: Self::Message) -> ShouldRender {
		false
	}

	fn view (&self) -> Html {
		let children = self.children.clone();
		let mut class = self.class.clone();
		let handle_click = &self.onclick;

		match self.kind {
			kind @ Kind::Anchor(_) => render("#", kind, class, Some(handle_click), children, false, self.disabled, self.loading, false),
			kind => {
				class.push_str(kind.class());

				if self.disabled
					{ class.push_str(" disabled"); }

				if self.loading
					{ class.push_str(" loading"); }

				if self.loading || self.disabled
					{ html!(<button class=class disabled=true>{children}</button>) }
				else if DEFAULT_HANDLER.with(|handler| handler.eq(handle_click))
					{ html!(<button type=self.effect.value() class=class>{children}</button>) }
				else
					{ html!(<button type=self.effect.value() class=class onclick=handle_click>{children}</button>) }
			}
		}
	}
}


thread_local! {
	static DEFAULT_HANDLER: Callback<MouseEvent> = Callback::from(|event: MouseEvent| event.prevent_default());
}
