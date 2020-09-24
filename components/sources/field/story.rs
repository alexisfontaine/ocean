use crate::story::*;
use super::*;


impl Story for Field {
	const TITLE: &'static str = "Field";


	fn gallery () -> Vec<Example> {
		vec![
			Example::from(html!(<Field kind=Field::Text label="Text" />))
		]
	}
}
