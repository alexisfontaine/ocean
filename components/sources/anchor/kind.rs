#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnchorModifier {
	Inline,
	Standalone,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonModifier {
	Danger,
	Default,
	Primary,
	Secondary,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
	Anchor(AnchorModifier),
	Button(ButtonModifier),
}


impl Kind {
	pub const fn class (&self) -> &'static str {
		match self {
			Self::Anchor(AnchorModifier::Inline) => "anchor--inline",
			Self::Anchor(AnchorModifier::Standalone) => "anchor",
			Self::Button(ButtonModifier::Danger) => "button--danger",
			Self::Button(ButtonModifier::Default) => "button",
			Self::Button(ButtonModifier::Primary) => "button--primary",
			Self::Button(ButtonModifier::Secondary) => "button--secondary",
		}
	}
}
