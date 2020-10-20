use proc_macro::TokenStream;
use quote::ToTokens;
use std::env::var;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use syn::LitStr;


pub struct Style<'a> {
	name: &'a LitStr,
	style: String
}


impl<'a> Style<'a> {
	pub fn new (name: &'a LitStr, mut style: String) -> Self {
		style.push_str("$root: ':root';");
		Self { name, style }
	}

	pub fn include (&self) {
		include(&self.name.value(), self.style.as_bytes());
	}

	pub fn to_token (&self) -> TokenStream {
		let name = self.name.value();

		include(&name, self.style.replace(":root", &format!(".{}", name)).as_bytes());
		self.name.to_token_stream().into()
	}
}


fn include (name: &str, style: &[u8]) {
	let mut file = OpenOptions::new()
		.create(true)
		.read(true)
		.write(true)
		.open(Path::new(&var("PACKAGE_DIR").unwrap()).join(name).with_extension("scss")).unwrap();

	if !Read::by_ref(&mut file).bytes().filter_map(Result::ok).eq(style.iter().copied()) {
		file.seek(SeekFrom::Start(0)).unwrap();
		file.set_len(style.len() as _).unwrap();
		file.write_all(style).unwrap();
	}
}
