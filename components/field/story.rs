use crate::story::*;
use super::*;


impl Story for Field {
	const TITLE: &'static str = "Field";


	fn gallery () -> Vec<Example> {
		vec![
			Example::with_title("Normal state", build_example(false, false)),
			Example::with_title("Disabled", build_example(true, false)),
			Example::with_title("Error", build_example(false, true)),
		]
	}
}


fn build_example (disabled: bool, error: bool) -> Vec<Vec<Html>> {
	vec![
		build_row(Field::Text, "Text", disabled, error),
	]
}

fn build_row (kind: Kind, label: &str, disabled: bool, error: bool) -> Vec<Html> {
	vec![
		html! {
			<Field kind=kind label=label disabled=disabled error=error />
		},

		html! {
			<Field kind=kind label={label.to_string() + " with details"} disabled=disabled error=error details="Some details" />
		},
	]
}
