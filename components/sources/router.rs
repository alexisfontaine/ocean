use yew::prelude::*;
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;

use super::components::anchor::{render, Kind, AnchorModifier, ButtonModifier};
use super::utils::ne_assign;


pub enum Message<STATE> {
	Navigate,
	Navigation(Route<STATE>),
}


pub struct Anchor<SWITCH, STATE = ()> where STATE: RouteState, SWITCH: Clone {
	handle_click: Callback<MouseEvent>,
	properties: Properties<SWITCH>,
	route: Option<SWITCH>,
	router: RouteAgentBridge<STATE>,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Properties<SWITCH> where SWITCH: Clone {
	pub children: Children,

	#[prop_or_default]
	pub class: String,

	#[prop_or_default]
	pub disabled: bool,

	#[prop_or(Kind::Anchor(AnchorModifier::Standalone))]
	pub kind: Kind,

	#[prop_or_default]
	pub loading: bool,

	pub route: SWITCH,
}


#[allow(non_upper_case_globals)]
impl<SWITCH, STATE> Anchor<SWITCH, STATE> where STATE: RouteState, SWITCH: Clone {
	pub const Button: Kind = Kind::Button(ButtonModifier::Default);

	pub const ButtonDanger: Kind = Kind::Button(ButtonModifier::Danger);

	pub const ButtonPrimary: Kind = Kind::Button(ButtonModifier::Primary);

	pub const ButtonSecondary: Kind = Kind::Button(ButtonModifier::Secondary);

	pub const Inline: Kind = Kind::Anchor(AnchorModifier::Inline);

	pub const Standalone: Kind = Kind::Anchor(AnchorModifier::Standalone);
}

impl<SWITCH, STATE> Component for Anchor<SWITCH, STATE> where STATE: RouteState + PartialEq, SWITCH: Clone + PartialEq + Switch + 'static {
	type Message = Message<STATE>;

	type Properties = Properties<SWITCH>;


	fn create (properties: Self::Properties, link: ComponentLink<Self>) -> Self {
		let mut router = RouteAgentBridge::new(link.callback(Self::Message::Navigation));

		router.send(RouteRequest::GetCurrentRoute);

		Self {
			properties,
			route: None,
			router,

			handle_click: link.callback(|event: MouseEvent| {
				event.prevent_default();
				Self::Message::Navigate
			}),
		}
	}

	fn change (&mut self, properties: Self::Properties) -> ShouldRender {
		ne_assign(&mut self.properties, properties)
	}

	fn update (&mut self, message: Self::Message) -> ShouldRender {
		match message {
			Self::Message::Navigate => self.router.send(RouteRequest::ChangeRoute(self.properties.route.clone().into())),
			Self::Message::Navigation(route) => {
				let route = SWITCH::switch(route);

				if route != self.route {
					self.route = route;

					return true
				}
			}
		}

		false
	}

	fn view (&self) -> Html {
		let properties = &self.properties;
		let route: Route<STATE> = properties.route.clone().into();

		render(route.as_str(), properties.kind, properties.class.clone(), Some(&self.handle_click), properties.children.clone(), self.route.contains(&properties.route), properties.disabled, properties.loading, false)
	}
}
