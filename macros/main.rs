#![feature(once_cell, proc_macro_span)]


#[cfg(feature = "code")]
mod code;

mod style;

#[cfg(feature = "code")]
use syn::parse_macro_input;

use proc_macro::TokenStream;
use std::fs::read_to_string;
use std::path::PathBuf;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{LitStr, Token};

#[cfg(feature = "code")]
use code::*;

use style::*;


#[cfg(feature = "code")]
#[proc_macro]
pub fn highlight (input: TokenStream) -> TokenStream {
	let input = Punctuated::<LitStr, Token![,]>::parse_separated_nonempty.parse(input).unwrap();

	assert_ne!(input.len(), 0);
	assert!(input.len() < 3);

	Code::new(input.first().unwrap().value(), input.last().map(LitStr::value).as_deref()).to_token()
}

#[cfg(feature = "code")]
#[proc_macro]
pub fn highlight_file (input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as LitStr);

	Code::to_token(&relative_path(&input).into())
}

#[proc_macro]
pub fn include_style (input: TokenStream) -> TokenStream {
	let input = Punctuated::<LitStr, Token![,]>::parse_separated_nonempty.parse(input).unwrap();

	assert_eq!(input.len(), 2);

	Style::new(input.first().unwrap(), input.last().unwrap().value()).include();
	TokenStream::new()
}

#[proc_macro]
pub fn include_style_file (input: TokenStream) -> TokenStream {
	let input = Punctuated::<LitStr, Token![,]>::parse_separated_nonempty.parse(input).unwrap();

	assert_eq!(input.len(), 2);

	Style::new(input.first().unwrap(), read_to_string(&relative_path(&input.last().unwrap())).unwrap()).include();
	TokenStream::new()
}


#[proc_macro]
pub fn style (input: TokenStream) -> TokenStream {
	let input = Punctuated::<LitStr, Token![,]>::parse_separated_nonempty.parse(input).unwrap();

	assert_eq!(input.len(), 2);

	Style::new(input.first().unwrap(), input.last().unwrap().value()).to_token()
}

#[proc_macro]
pub fn style_file (input: TokenStream) -> TokenStream {
	let input = Punctuated::<LitStr, Token![,]>::parse_separated_nonempty.parse(input).unwrap();

	assert_eq!(input.len(), 2);

	Style::new(input.first().unwrap(), read_to_string(&relative_path(&input.last().unwrap())).unwrap()).to_token()
}


fn relative_path (input: &LitStr) -> PathBuf {
	let mut path = input.span().unwrap().source_file().path();

	path.pop();
	path.push(input.value());
	path
}
