use proc_macro::TokenStream;
use quote::ToTokens;
use std::fs::read_to_string;
use std::lazy::SyncLazy;
use std::path::{Path, PathBuf};
use syntect::html::{ClassedHTMLGenerator, ClassStyle};
use syntect::parsing::{SyntaxReference, SyntaxSet};


pub struct Code<'a> {
	source: String,
	syntax: &'a SyntaxReference,
}


impl Code<'_> {
	pub fn new (source: String, syntax: Option<&str>) -> Self {
		Self {
			source,
			syntax: syntax.and_then(|token| SYNTAX.find_syntax_by_token(token)).unwrap_or(&*DEFAULT_SYNTAX),
		}
	}

	pub fn highlight (&self) -> TokenStream {
		let mut generator = ClassedHTMLGenerator::new_with_class_style(&self.syntax, &*SYNTAX, ClassStyle::Spaced);

		for line in self.source.lines()
			{ generator.parse_html_for_line(line); }

		generator.finalize().into_token_stream().into()
	}
}


impl<T> From<T> for Code<'_> where T: AsRef<Path> {
	fn from (path: T) -> Self {
		let path = PathBuf::from(path.as_ref());

		Self {
			source: read_to_string(&path).unwrap(),
			syntax: path.extension().and_then(|extension| SYNTAX.find_syntax_by_extension(extension.to_str()?)).unwrap_or(&*DEFAULT_SYNTAX),
		}
	}
}


static SYNTAX: SyncLazy<SyntaxSet> = SyncLazy::new(SyntaxSet::load_defaults_newlines);

static DEFAULT_SYNTAX: SyncLazy<&SyntaxReference> = SyncLazy::new(|| SYNTAX.find_syntax_plain_text());
