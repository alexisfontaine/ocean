use crate::code::{highlight, Code};
use crate::story::*;
use super::*;


impl Story for Anchor {
	const TITLE: &'static str = "Anchor";


	fn gallery () -> Vec<Example> {
		vec! [
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
			<p>{"The "}<Code inline=true content=Self::TITLE />{" component is used as a navigational element. They usually contains a label and/or icons and may either resemble a button or an anchor."}</p>
		})
	}
}


fn build_example (disabled: bool, loading: bool) -> Vec<Vec<Html>> {
	let mut rows = if loading
		{ Vec::new() } else
		{ vec![build_row("Inline", Anchor::Inline, disabled, loading)] };

	rows.append(&mut vec![
		build_row("Standalone", Anchor::Standalone, disabled, loading),
		build_row("Default", Anchor::Button, disabled, loading),
		build_row("Primary", Anchor::ButtonPrimary, disabled, loading),
		build_row("Secondary", Anchor::ButtonSecondary, disabled, loading),
		build_row("Danger", Anchor::ButtonDanger, disabled, loading),
	]);

	rows
}

fn build_row (label: &str, kind: Kind, disabled: bool, loading: bool) -> Vec<Html> {
	vec! [
		html! {
			<Anchor to="#" kind=kind disabled=disabled loading=loading>{label}</Anchor>
		},

		html! {
			<Anchor to="#" kind=kind disabled=disabled loading=loading>
				{icon::back()}
				<span>{"Return to some page"}</span>
			</Anchor>
		},

		html! {
			<Anchor to="#" kind=kind disabled=disabled loading=loading>
				<span>{"Open some link"}</span>
				{icon::open()}
			</Anchor>
		},

		html! {
			<Anchor to="#" kind=kind disabled=disabled loading=loading>{icon::link()}</Anchor>
		},
	]
}

