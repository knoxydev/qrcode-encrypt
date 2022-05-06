pub mod creator {
	use qrcode_png::*;

	pub fn generate(txt: &str) {
		let mut qrcode = QrCode::new(txt, QrCodeEcc::Medium).unwrap();

		qrcode.margin(10);
		qrcode.zoom(10);

		let buf = qrcode.generate(Color::Grayscale(0, 255)).unwrap();
		std::fs::write("./qrcode.png", buf).unwrap();
	}
}