pub mod icon;
pub mod text;

mod example;


use yew::prelude::*;

pub use example::Example;

use crate::title::Title;

pub trait Story {
	const TITLE: &'static str;


	fn gallery () -> Vec<Example> {
		Vec::new()
	}

	fn notes () -> Option<Html> {
		None
	}

	fn overview () -> Option<Html> {
		None
	}

	fn usage () -> Option<Html> {
		None
	}

	fn render () -> Html {
		html! {
			<>
				<Title>{Self::TITLE}</Title>

				{render_section("Overview", Self::overview())}
				{render_section("Gallery", Self::gallery().into_iter().map(Example::render))}
				{render_section("Usage", Self::usage())}
				{render_section("Notes", Self::notes())}
			</>
		}
	}
}


fn render_section<T> (title: &str, content: T) -> Html where T: IntoIterator<Item = Html>, T::IntoIter: ExactSizeIterator {
	let content = content.into_iter();

	if content.len() == 0 {
		html! {}
	} else {
		html! {
			<article>
				<Title kind=Title::Section>{title}</Title>

				{for content}
			</article>
		}
	}
}
