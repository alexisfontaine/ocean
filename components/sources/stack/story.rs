use crate::code::Code;
use crate::story::*;
use crate::title::Title;
use super::*;


impl Story for Stack {
	const TITLE: &'static str = "Stack";


	fn gallery() -> Vec<Example> {
		vec! [
			Example::from(html! {
				<Stack>
					<article>
						<Title kind=Title::Section>{"First section"}</Title>

						<Title kind=Title::SubSection>{"First paragraph"}</Title>

						{text::paragraph_1()}

						<Title kind=Title::SubSection>{"Second paragraph"}</Title>

						{text::paragraph_2()}
					</article>

					<article>
						<Title kind=Title::Section>{"Second section"}</Title>

						{text::paragraph_1()}
					</article>
				</Stack>
			}),
		]
	}

	fn overview() -> Option<Html> {
		Some(html! {
			<p>{"The "}<Code inline=true content=Self::TITLE />{" component is a container element used to make a single column layout."}</p>
		})
	}
}
