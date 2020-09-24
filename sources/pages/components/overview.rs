use ocean::Title;
use ocean::story::Story;
use yew::prelude::*;


pub struct Overview {}


impl Story for Overview {
	const TITLE: &'static str = "Overview";


	fn render () -> Html {
		html! {
			<>
				<Title>{Self::TITLE}</Title>

				<article>
					{"overview"}
				</article>
			</>
		}
	}
}
