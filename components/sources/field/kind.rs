#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
	Text,
}


impl Kind {
	pub const fn value (&self) -> &'static str {
		match self {
			Self::Text => "text",
		}
	}
}
