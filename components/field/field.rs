mod kind;

#[cfg(feature = "story")]
mod story;


pub use self::kind::*;

use std::cell::Cell;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::utils::ne_assign;


#[derive(Debug)]
pub enum Message {
	Update(String)
}


#[derive(Clone, Debug)]
pub struct Field {
	handle_input: Callback<InputData>,
	identifier: String,
	input: NodeRef,
	properties: Properties,
	value: String
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Properties {
	#[prop_or_default]
	pub autofocus: bool,

	#[prop_or_default]
	pub class: String,

	#[prop_or_default]
	pub details: String,

	#[prop_or_default]
	pub disabled: bool,

	#[prop_or_default]
	pub error: bool,

	#[prop_or(Kind::Text)]
	pub kind: Kind,

	pub label: String,

	#[prop_or_else(|| DEFAULT_HANDLER.with(|handler| (*handler).clone()))]
	pub oninput: Callback<String>
}


#[allow(non_upper_case_globals)]
impl Field {
	pub const Text: Kind = Kind::Text;
}


impl Component for Field {
	type Message = Message;

	type Properties = Properties;


	fn create (properties: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			handle_input: link.callback(|event: InputData| Self::Message::Update(event.value)),
			identifier: format!("{:x}", COUNTER.with(|counter| counter.update(|value| value + 1))),
			input: NodeRef::default(),
			properties,
			value: String::new()
		}
	}

	fn change (&mut self, properties: Self::Properties) -> ShouldRender {
		ne_assign(&mut self.properties, properties)
	}

	fn rendered (&mut self, first_render: bool) {
		if first_render && self.properties.autofocus
			{ self.input.cast::<HtmlInputElement>().unwrap().focus().unwrap(); }
	}

	fn update (&mut self, message: Self::Message) -> ShouldRender {
		match message {
			Self::Message::Update(value) => {
				if value == self.value
					{ return false }

				self.value = value.clone();
				self.properties.oninput.emit(value);
			}
		}

		true
	}

	fn view (&self) -> Html {
		let properties = &self.properties;
		let mut class = properties.class.clone();
		let details = &properties.details;

		if class.is_empty()
			{ class.push_str("field"); }

		if properties.error
			{ class.push_str(" error"); }

		html! {
			// `<fieldset>` has a bug on Chrome that prevents it from using `flex` or `grid` layout.
			<div class=class>
				<input autofocus=properties.autofocus disabled=properties.disabled id=&self.identifier oninput=&self.handle_input type=properties.kind.value() ref=self.input.clone() />

				<label for=&self.identifier>{&properties.label}</label>

				{for if details.is_empty() { None } else { Some(html!(<span>{details}</span>)) }}
			</div>
		}
	}
}


thread_local! {
	static COUNTER: Cell<u32> = Cell::default();

	static DEFAULT_HANDLER: Callback<String> = Callback::noop();
}
