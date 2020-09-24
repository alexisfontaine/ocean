use ocean::router::Anchor;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::switch::Permissive;

use super::pages::*;


pub type ApplicationAnchor = Anchor<ApplicationRoute>;


#[derive(Clone, Debug, PartialEq, Switch)]
pub enum ApplicationRoute {
	#[to = "/!"]
	Home,

	#[to = "/about!"]
	About,

	#[to = "/components!"]
	Components,

	#[to = "/components/{*}"]
	Component(ComponentRoute),

	#[to = "/documentation"]
	Documentation,

	#[to = "/not-found!"]
	NotFound(Permissive<String>),
}


impl ApplicationRoute {
	pub fn redirect (route: Route<()>) -> Self {
		Self::NotFound(Permissive(Some(route.route)))
	}

	pub fn in_presentation_mode (&self) -> bool {
		matches!(self, Self::About | Self::Home | Self::NotFound(_))
	}

	pub fn render (&self) -> Html {
		match self {
			Self::About => html!(<About />),
			Self::Component(component) => html!(<Components component=component />),
			Self::Components => html!(<Components />),
			Self::Documentation => html!(<Documentation />),
			Self::Home => html!(<Home />),
			Self::NotFound(_) => html!(<NotFound />),
		}
	}
}
