#![allow(warnings)]

// PACKAGES
use std::path::Path;

// MODULES
mod read;
pub use crate::read::reader;

mod create;
pub use crate::create::creator;


fn input() {
	let action = std::env::args().nth(1).expect("no pattern given");
	
	if action == "create" {

		let encode = std::env::args().nth(2).expect("no pattern given");
		let key = std::env::args().nth(3).expect("no pattern given");
		let text = std::env::args().nth(4).expect("no pattern given");
		creator::start(&encode, &key, &text);

	} else if action == "scan" {

		let encode = std::env::args().nth(2).expect("no pattern given");
		let key = std::env::args().nth(3).expect("no pattern given");
		let fileph = std::env::args().nth(4).expect("no pattern given");

		if Path::new(&fileph).exists() == true { println!("{:?}", reader::scan(&fileph, &key, &encode).unwrap()); }
		else { println!("Incorrect file name or file path :("); }

	} else {
		println!("Incorrect command !");
		return;
	}
}

fn main() { input(); }

