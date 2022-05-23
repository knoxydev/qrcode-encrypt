pub mod reader {
	use std::path::Path;
	use image;
	use std::error::Error;
	use rqrr::PreparedImage;
	extern crate cipher_crypt;
	use cipher_crypt::{Cipher, Rot13, Caesar, Vigenere, Porta};

	use std::fs::File;
	use std::io::Write;

	pub fn scan(filepath: &str, key: &str, encode: &str) -> Result<(), Box<dyn Error>> {
		let img = image::open(filepath)?.to_luma8();
		let mut prepared_img = PreparedImage::prepare(img);

		let grids = prepared_img.detect_grids();
		let contents = grids.into_iter().map(|grid| {
			let (_, content) = grid.decode().unwrap_or_else(|err| {
			eprintln!("\nERROR reading data from qr code: {}", err);
			panic!();
		});
		content}).collect::<Vec<String>>();

		let mut res = String::new();

		// CHECKING KEY
		let base_one = ["base64", "hex", "txt", "morse", "rot13"];
		let base_two = ["caesar", "vigenere", "porta"];

		let mut exit_one: bool = false;
		let mut exit_two: bool = false;

		for i in base_one {
			if encode == i {
				if key != "--key" {
					exit_one = true;
					break;
				}
			}
		}
		for i in base_two {
			if encode == i {
				if key == "--key" {
					exit_two = true;
					break;
				}
			}
		}

		if exit_one == true {
			println!("This method doesn't require a key");
			return Ok(())
		}
		if exit_two == true {
			println!("This method requires a key");
			return Ok(())
		}
		// CHECKING KEY
		
		if encode == "base64" {
			let x = base64::decode(contents[0].to_string()).unwrap();
			res = String::from_utf8_lossy(&x).to_string();
		}
		else if encode == "hex" { 
			let x = hex::decode(contents[0].to_string());
			res = String::from_utf8_lossy(&x.unwrap()).to_string();
		}
		else if encode == "morse" {
			let x = &crypto_morse::decode(&contents[0]);
			res = x.to_string();
		}
		else if encode == "rot13" {
			let x = Rot13::decrypt(&contents[0]);
			res = x.to_string();
		}
		else if encode == "txt" { res = contents[0].to_string(); }
		else if encode == "caesar" {
			let num = key.parse::<i64>().unwrap();
			let x = Caesar::new(num.try_into().unwrap());
			res = x.decrypt(&contents[0]).unwrap();
		}
		else if encode == "vigenere" {
			let x = Vigenere::new((&key).to_string());
			res = x.decrypt(&contents[0]).unwrap();
		}
		else if encode == "porta" {
			let x = Porta::new((&key).to_string());
			res = x.decrypt(&contents[0]).unwrap();
		}
		else { println!("Incorrect encode method"); }

		let mut output = File::create("result-qrcode.txt")?;
    write!(output, "{}", res)?;

		Ok(())
	}
}