use ocean::{Stack, Title};
use yew::prelude::*;


pub struct NotFound;


impl Component for NotFound {
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
				<article>
					<Title>{"Not Found"}</Title>
				</article>
			</Stack>
		}
	}
}
