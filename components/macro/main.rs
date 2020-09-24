#![feature(once_cell)]


use proc_macro::TokenStream;
use quote::ToTokens;
use std::fs::read_to_string;
use std::lazy::SyncLazy;
use std::path::PathBuf;
use syn::{parse_macro_input, LitStr, Token};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syntect::html::{ClassedHTMLGenerator, ClassStyle};
use syntect::parsing::{SyntaxReference, SyntaxSet};


struct Code<'a> {
	source: String,
	syntax: &'a SyntaxReference,
}


impl Code<'_> {
	fn highlight (&self) -> TokenStream {
		let mut generator = ClassedHTMLGenerator::new_with_class_style(&self.syntax, &*SYNTAX, ClassStyle::Spaced);

		for line in self.source.lines()
			{ generator.parse_html_for_line(line); }

		generator.finalize().into_token_stream().into()
	}
}


#[proc_macro]
pub fn highlight (input: TokenStream) -> TokenStream {
	let input = Punctuated::<LitStr, Token![,]>::parse_separated_nonempty.parse(input).unwrap();

	assert!(input.len() < 3);

	Code {
		source: input.first().unwrap().value(),
		syntax: input.last().and_then(|token| SYNTAX.find_syntax_by_token(&token.value())).unwrap_or(&*DEFAULT_SYNTAX),
	}.highlight()
}

#[proc_macro]
pub fn highlight_file (input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as LitStr);
	let path = PathBuf::from(input.value());

	Code {
		source: read_to_string(&path).unwrap(),
		syntax: path.extension().and_then(|extension| SYNTAX.find_syntax_by_extension(extension.to_str()?)).unwrap_or(&*DEFAULT_SYNTAX),
	}.highlight()
}


static SYNTAX: SyncLazy<SyntaxSet> = SyncLazy::new(|| SyntaxSet::load_defaults_newlines());

static DEFAULT_SYNTAX: SyncLazy<&SyntaxReference> = SyncLazy::new(|| SYNTAX.find_syntax_plain_text());
