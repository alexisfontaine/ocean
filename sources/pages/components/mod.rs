mod overview;


use ocean::components::*;
use ocean::story::Story;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::{ApplicationAnchor, ApplicationRoute};
use overview::Overview;


#[derive(Clone, Debug, PartialEq, Switch)]
pub enum ComponentRoute {
	#[to = "action!"]
	Action,

	#[to = "anchor!"]
	Anchor,

	#[to = "code!"]
	Code,

	#[to = "field!"]
	Field,

	#[to = "list!"]
	List,

	#[to = "!"]
	Overview,

	#[to = "stack!"]
	Stack,

	#[to = "title!"]
	Title,
}


pub struct Components {
	properties: Properties,
	slot: Html,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Properties {
	#[prop_or(ComponentRoute::Overview)]
	pub component: ComponentRoute,
}


impl ComponentRoute {
	fn render (&self) -> Html {
		match self {
			Self::Action => <Action as Story>::render(),
			Self::Anchor => <Anchor as Story>::render(),
			Self::Code => <Code as Story>::render(),
			Self::Field => <Field as Story>::render(),
			Self::List => <List as Story>::render(),
			Self::Overview => Overview::render(),
			Self::Stack => <Stack as Story>::render(),
			Self::Title => <Title as Story>::render(),
		}
	}
}

impl Component for Components {
	type Message = ();

	type Properties = Properties;


	fn create (properties: Self::Properties, _: ComponentLink<Self>) -> Self {
		let slot = properties.component.render();

		Self { properties, slot }
	}

	fn change (&mut self, properties: Self::Properties) -> ShouldRender {
		if self.properties == properties {
			false
		} else {
			let slot = properties.component.render();

			self.properties = properties;
			self.slot = slot;
			true
		}
	}

	fn update (&mut self, _: Self::Message) -> ShouldRender {
		false
	}

	fn view (&self) -> Html {
		html! {
			<>
				<List tag="nav" class="application__sidebar">
					<ApplicationAnchor route=ApplicationRoute::Components>{Overview::TITLE}</ApplicationAnchor>

					<ApplicationAnchor route=ApplicationRoute::Component(ComponentRoute::Action)>{Action::TITLE}</ApplicationAnchor>
					<ApplicationAnchor route=ApplicationRoute::Component(ComponentRoute::Anchor)>{Anchor::TITLE}</ApplicationAnchor>
					<ApplicationAnchor route=ApplicationRoute::Component(ComponentRoute::Code)>{Code::TITLE}</ApplicationAnchor>
					<ApplicationAnchor route=ApplicationRoute::Component(ComponentRoute::Field)>{Field::TITLE}</ApplicationAnchor>
					<ApplicationAnchor route=ApplicationRoute::Component(ComponentRoute::List)>{List::TITLE}</ApplicationAnchor>
					<ApplicationAnchor route=ApplicationRoute::Component(ComponentRoute::Stack)>{Stack::TITLE}</ApplicationAnchor>
					<ApplicationAnchor route=ApplicationRoute::Component(ComponentRoute::Title)>{Title::TITLE}</ApplicationAnchor>
				</List>

				<Stack tag="main">{self.slot.clone()}</Stack>
			</>
		}
	}
}
