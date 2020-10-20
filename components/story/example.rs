use yew::prelude::*;

use crate::title::Title;
use crate::utils::style_file;


pub struct Example {
	content: Vec<Vec<Html>>,
	title: Option<&'static str>,
}


impl Example {
	pub const STYLE: &'static str = style_file!("example", "./example.scss");


	pub fn new (content: Vec<Vec<Html>>) -> Self {
		Self {
			content,
			title: None,
		}
	}

	pub fn with_title (title: &'static str, content: Vec<Vec<Html>>) -> Self {
		Self {
			content,
			title: Some(title)
		}
	}

	pub fn render (self) -> Html {
		let content = self.content;
		let style = format!("--columns: {};", content[0].len());

		html! {
			<>
				<Title kind=Title::SubSection>{for self.title}</Title>

				<section class=Self::STYLE style=style>{for content.into_iter().flatten()}</section>
			</>
		}
	}
}

impl From<Html> for Example {
	fn from (content: Html) -> Self {
		vec![content].into()
	}
}

impl From<Vec<Html>> for Example {
	fn from (content: Vec<Html>) -> Self {
		Self::new(vec![content])
	}
}

impl From<(&'static str, Html)> for Example {
	fn from (content: (&'static str, Html)) -> Self {
		(content.0, vec![content.1]).into()
	}
}

impl From<(&'static str, Vec<Html>)> for Example {
	fn from (content: (&'static str, Vec<Html>)) -> Self {
		Self::with_title(content.0, vec![content.1])
	}
}
