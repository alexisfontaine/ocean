use std::env::var;
use std::fs::create_dir_all;


fn main () {
	let package = var("PACKAGE_DIR").unwrap();

	create_dir_all(&package).unwrap();
	println!("cargo:rustc-env=PACKAGE={}", package);
}
