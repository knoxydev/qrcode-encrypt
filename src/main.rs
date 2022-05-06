#![allow(warnings)]

use std::path::Path;
extern crate base64;

// MODULES
mod read;
pub use crate::read::reader;

mod create;
pub use crate::create::creator;


fn input() {
	let action = std::env::args().nth(1).expect("no pattern given");
	
	if action == "create" {

		let encode = std::env::args().nth(2).expect("no pattern given");
		let text = std::env::args().nth(3).expect("no pattern given");

		if encode == "base64" { creator::generate(&base64::encode(text.into_bytes())); }
		else if encode == "hex" { creator::generate(&hex::encode(text)); }
		else if encode == "txt" { creator::generate(&text);}
		else { println!("Incorrect encode method"); }

	} else if action == "scan" {
		let encode = std::env::args().nth(2).expect("no pattern given");
		let fileph = std::env::args().nth(3).expect("no pattern given");

		if Path::new(&fileph).exists() == true { println!("{:?}", reader::scan(&fileph, &encode).unwrap()); }
		else { println!("Incorrect file name or file path :("); }

	} else {
		println!("Incorrect command !");
		return;
	}
}

fn main() { input(); }

