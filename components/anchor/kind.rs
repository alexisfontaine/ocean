use crate::utils::style_file;


macro_rules! style {
	("anchor") => {style_file!("anchor", "components/anchor/anchor.scss")};
	("action") => {style_file!("action", "components/action/action.scss")};
}


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
			Self::Anchor(AnchorModifier::Inline) => concat!(style!("anchor"), "--inline"),
			Self::Anchor(AnchorModifier::Standalone) => style!("anchor"),
			Self::Button(ButtonModifier::Danger) => concat!(style!("action"), "--danger"),
			Self::Button(ButtonModifier::Default) => style!("action"),
			Self::Button(ButtonModifier::Primary) => concat!(style!("action"), "--primary"),
			Self::Button(ButtonModifier::Secondary) => concat!(style!("action"), "--secondary"),
		}
	}
}
