pub mod creator {

	use qrcode_png::*;
	extern crate base64;
	extern crate cipher_crypt;
	use cipher_crypt::{Cipher, Rot13, Caesar};
	// OTHER PACKAGES: hex, crypto_morse

	fn generate(txt: &str) {
		let mut qrcode = QrCode::new(txt, QrCodeEcc::Medium).unwrap();

		qrcode.margin(12);
		qrcode.zoom(12);

		let buf = qrcode.generate(Color::Grayscale(0, 255)).unwrap();
		std::fs::write("./qrcode.png", buf).unwrap();
	}

	pub fn start(encd: &str, txt: &str) {
		if encd == "base64" { generate(&base64::encode(String::from(txt).into_bytes())); }
		else if encd == "hex" { generate(&hex::encode(txt)); }
		else if encd == "txt" { generate(&txt);}
		else if encd == "morse" { generate(&crypto_morse::encode(&txt)); }
		else if encd == "rot13" { generate(&Rot13::encrypt(&txt)); }
		else if encd == "caesar" {
			let c = Caesar::new(4);
			generate(&c.encrypt(&txt).unwrap());
		}
		else { println!("Incorrect encode method"); }
	}
}