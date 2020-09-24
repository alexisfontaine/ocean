use crate::story::*;
use super::*;


impl Story for Title {
	const TITLE: &'static str = "Title";


	fn gallery () -> Vec<Example> {
		vec![
			Example::from(html! {
				<p>
					<Title kind=Title::Headline>{"Headline"}</Title>
					<Title kind=Title::Main>{"Main"}</Title>
					<Title kind=Title::Section>{"Section"}</Title>
					<Title kind=Title::SubSection>{"Sub section"}</Title>

					{text::paragraph_1()}
					{text::paragraph_2()}
				</p>
			})
		]
	}
}
