use crate::story::*;
use super::*;


impl Story for Code {
	const TITLE: &'static str = "Code";


	fn gallery () -> Vec<Example> {
		vec! [
			Example::from(("Inline", html! {
				<p>
					{text::TEXT_1}
					{" "}
					<Code inline=true content=highlight!("<Code inline=true content=highlight!(\"[...]\") />", "rust") />
					{" "}
					{text::TEXT_2}
				</p>
			})),

			Example::from(("Block", html! {
				<Code content=highlight_file!("components/sources/code/story.rs") />
			})),
		]
	}

	fn notes () -> Option<Html> {
		Some(html! {
			<ul>
				<li>{"The "}<Code inline=true content="content" />{" property will render unescaped HTML."}</li>
			</ul>
		})
	}
}
