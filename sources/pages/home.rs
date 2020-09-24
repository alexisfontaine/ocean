use ocean::{Anchor, Stack, Title};
use yew::prelude::*;

use crate::route::{ApplicationAnchor, ApplicationRoute};


pub struct Home;


impl Component for Home {
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
			<Stack tag="main" class="stretched">
				<article class="hero">
					<Title kind=Title::Headline>{"Ocean Design System"}</Title>

					<p>
						{"An experimental design system made for fun with "}
						<Anchor kind=Anchor::Inline to="https://rust-lang.org/" open_in_new_context=true>{"Rust"}</Anchor>
						{" front-end framework "}
						<Anchor kind=Anchor::Inline to="https://yew.rs" open_in_new_context=true>{"Yew"}</Anchor>
						{"."}
					</p>

					<ApplicationAnchor kind=ApplicationAnchor::ButtonSecondary route=ApplicationRoute::Components>{"Get started"}</ApplicationAnchor>
				</article>
			</Stack>
		}
	}
}
