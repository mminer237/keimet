use copy_to_output::copy_to_output;
use std::env;

fn main() {
	let _ = copy_to_output("bibles", &env::var("PROFILE").unwrap());
}