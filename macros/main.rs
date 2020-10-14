#![feature(once_cell)]


mod code;

use proc_macro::TokenStream;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, LitStr, Token};

use code::*;


#[proc_macro]
pub fn highlight (input: TokenStream) -> TokenStream {
	let input = Punctuated::<LitStr, Token![,]>::parse_separated_nonempty.parse(input).unwrap();

	assert_ne!(input.len(), 0);
	assert!(input.len() < 3);

	Code::new(input.first().unwrap().value(), input.last().map(LitStr::value).as_deref()).highlight()
}

#[proc_macro]
pub fn highlight_file (input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as LitStr);

	Code::highlight(&input.value().into())
}
