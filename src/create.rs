pub mod creator {

	use qrcode_png::*;
	extern crate base64;
	extern crate cipher_crypt;
	use cipher_crypt::{Cipher, Rot13, Caesar, Vigenere, Porta, Scytale};
	// OTHER PACKAGES: hex, crypto_morse

	fn generate(txt: &str) {
		let mut qrcode = QrCode::new(txt, QrCodeEcc::Medium).unwrap();

		qrcode.margin(12);
		qrcode.zoom(12);

		let buf = qrcode.generate(Color::Grayscale(0, 255)).unwrap();
		std::fs::write("./qrcode.png", buf).unwrap();
	}

	pub fn start(encd: &str, key: &str, txt: &str) {
		// CHECKING KEY
		let base_one = ["base64", "hex", "txt", "morse", "rot13"];
		let base_two = ["caesar", "vigenere", "porta", "scytale"];

		let mut exit_one: bool = false;
		let mut exit_two: bool = false;

		for i in base_one {
			if encd == i {
				if key != "--key" {
					exit_one = true;
					break;
				}
			}
		}
		for i in base_two {
			if encd == i {
				if key == "--key" {
					exit_two = true;
					break;
				}
			}
		}

		if exit_one == true { return println!("This method doesn't require a key"); }
		if exit_two == true { return println!("This method requires a key"); }
		// CHECKING KEY

		if encd == "base64" { generate(&base64::encode(String::from(txt).into_bytes())); }
		else if encd == "hex" { generate(&hex::encode(txt)); }
		else if encd == "txt" { generate(&txt); }
		else if encd == "morse" { generate(&crypto_morse::encode(&txt)); }
		else if encd == "rot13" { generate(&Rot13::encrypt(&txt)); }
		else if encd == "caesar" {
			let num = key.parse::<i64>().unwrap();
			let k = Caesar::new(num.try_into().unwrap());
			generate(&k.encrypt(&txt).unwrap());
		}
		else if encd == "vigenere" {
			let k = Vigenere::new((&key).to_string());
			generate(&k.encrypt(&txt).unwrap());
		}
		else if encd == "porta" {
			let k = Porta::new((&key).to_string());
			generate(&k.encrypt(&txt).unwrap());
		}
		else if encd == "scytale" {
			let num = key.parse::<i64>().unwrap();
			let k = Scytale::new(num.try_into().unwrap());
			generate(&k.encrypt(&txt).unwrap());
		}
		else { println!("Incorrect encode method"); }
	}
}