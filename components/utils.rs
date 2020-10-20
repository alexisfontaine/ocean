pub use ocean_macros::{include_style, include_style_file, style, style_file};


#[macro_export]
macro_rules! conditional {
	($enabled:expr, $value:expr) => (if $enabled { Some($value) } else { None });
}


pub fn ne_assign<T> (a: &mut T, b: T) -> bool where T: PartialEq {
	if &b == a {
		false
	} else {
		*a = b;
		true
	}
}
