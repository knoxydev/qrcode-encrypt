pub mod creator {
	use qrcode_png::*;

	pub fn generate() {
		let mut qrcode = QrCode::new(b"Hello Rust ! Iam NKR413", QrCodeEcc::Medium).unwrap();

		qrcode.margin(10);
		qrcode.zoom(10);

		let buf = qrcode.generate(Color::Grayscale(0, 255)).unwrap();
		std::fs::write("./qrcode.png", buf).unwrap();
	}
}