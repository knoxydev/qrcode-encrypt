#![allow(warnings)]

use std::path::Path;

// MODULES
mod read;
pub use crate::read::reader;

mod create;
pub use crate::create::creator;


fn input() {
	println!("\nEnter command 'scan/create/help' -> ");

	let mut resp = String::new();
	std::io::stdin().read_line(&mut resp).expect("Failes");
	let rsp = &resp[0..&resp.len() - 2].to_string();

	if rsp == "/scan" {
		println!("\nEnter file name -> ");

		let mut new_resp = String::new();
		std::io::stdin().read_line(&mut new_resp).expect("Failes");
		let fileph = &new_resp[0..&new_resp.len() - 2].to_string();

		if Path::new(&fileph).exists() == true { println!("{:?}", reader::scan(&fileph).unwrap()); }
		else { println!("Incorrect file name or file path :("); }
	}

	else if rsp == "/create" { creator::generate(); }

	else if rsp == "/help" { }

	else if rsp == "/exit" {
		println!("Exit !");
		return;
	}

	else { println!("none"); }

	input();
}

fn main() { input(); }

