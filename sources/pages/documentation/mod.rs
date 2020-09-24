use ocean::{Stack, Title};
use yew::prelude::*;

use crate::route::{ApplicationAnchor, ApplicationRoute};


pub struct Documentation;


impl Component for Documentation {
	type Message = ();

	type Properties = ();


	fn create (_: Self::Properties, _: ComponentLink<Self>) -> Self {
		Self
	}

	fn change (&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn update (&mut self, _: Self::Message) -> ShouldRender {
		false
	}

	fn view (&self) -> Html {
		html! {
			<>
				<nav class="application__sidebar list">
					<ApplicationAnchor route=ApplicationRoute::Documentation>{"Introduction"}</ApplicationAnchor>
				</nav>

				<Stack tag="main">
					<article>
						<Title>{"Documentation"}</Title>
					</article>
				</Stack>
			</>
		}
	}
}
