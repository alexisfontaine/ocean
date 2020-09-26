use crate::code::{highlight, Code};
use crate::story::*;
use super::*;


impl Story for Action {
	const TITLE: &'static str = "Action";


	fn gallery () -> Vec<Example> {
		vec![
			Example::with_title("Normal state", build_example(false, false)),
			Example::with_title("Disabled", build_example(true, false)),
			Example::with_title("Loading", build_example(false, true)),
		]
	}

	fn notes () -> Option<Html> {
		Some(html! {
			<ul>
				<li>{"Wrap the label in a "}<Code inline=true content=highlight!("<span>", "html") />{" element to make the "}<Code inline=true content=highlight!("text-overflow", "css") />{" CSS property work."}</li>
			</ul>
		})
	}

	fn overview () -> Option<Html> {
		Some(html! {
			<p>{"The "}<Code inline=true content=Self::TITLE />{" component is a clickable element used to initialize an action. They usually contains a label and/or icons and may either resemble a button or an anchor."}</p>
		})
	}
}


fn build_example (disabled: bool, loading: bool) -> Vec<Vec<Html>> {
	let mut rows = if loading
		{ Vec::new() } else
		{ vec![build_row("Inline", Action::AnchorInline, disabled, loading)] };

	rows.append(&mut vec![
		build_row("Standalone", Action::Anchor, disabled, loading),
		build_row("Default", Action::Button, disabled, loading),
		build_row("Primary", Action::ButtonPrimary, disabled, loading),
		build_row("Secondary", Action::ButtonSecondary, disabled, loading),
		build_row("Danger", Action::ButtonDanger, disabled, loading),
	]);

	rows
}

fn build_row (label: &str, kind: Kind, disabled: bool, loading: bool) -> Vec<Html> {
	vec![
		html! {
			<Action kind=kind disabled=disabled loading=loading>{label}</Action>
		},

		html! {
			<Action kind=kind disabled=disabled loading=loading>
				{icon::star()}
				<span>{"Star"}</span>
			</Action>
		},

		html! {
			<Action kind=kind disabled=disabled loading=loading>
				<span>{"Add some item"}</span>
				{icon::add()}
			</Action>
		},

		html! {
			<Action kind=kind disabled=disabled loading=loading>{icon::settings()}</Action>
		},
	]
}
