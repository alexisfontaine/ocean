use std::env::var;
use std::fs::create_dir_all;


fn main () {
	if let Ok(directory) = var("PACKAGE_DIR")
		{ create_dir_all(&directory).unwrap(); }
}
