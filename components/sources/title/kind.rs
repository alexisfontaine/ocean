#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
	Headline,
	Main,
	Section,
	SubSection,
}


impl Kind {
	pub const fn class (&self) -> &'static str {
		match self {
			Self::Headline => "title--headline",
			Self::Main => "title",
			Self::Section => "title--section",
			Self::SubSection => "title--sub-section",
		}
	}

	pub const fn tag (&self) -> &'static str {
		match self {
			Self::Headline => "h1",
			Self::Main => "h1",
			Self::Section => "h2",
			Self::SubSection => "h3",
		}
	}
}
