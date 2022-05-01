#![allow(warnings)]

use qrcode_png::*;
use image;
use std::error::Error;
use rqrr::PreparedImage;


type BoxResult<T> = Result<T, Box<dyn Error>>;
fn read() -> BoxResult<Vec<String>> {
	let img = image::open("./qrcode.png")?.to_luma8();
	let mut prepared_img = PreparedImage::prepare(img);

	let grids = prepared_img.detect_grids();
	let contents = grids.into_iter().map(|grid| {
		let (_, content) = grid.decode().unwrap_or_else(|err| {
		eprintln!("\nERROR reading data from qr code: {}", err);
		panic!();
	});
	content}).collect::<Vec<String>>();
		
	Ok(contents)
}

fn generate() {
	let mut qrcode = QrCode::new(b"Hello Rust ! Iam NKR413", QrCodeEcc::Medium).unwrap();

	qrcode.margin(10);
	qrcode.zoom(10);

	let buf = qrcode.generate(Color::Grayscale(0, 255)).unwrap();
	std::fs::write("./qrcode.png", buf).unwrap();
}

fn main() {
	let x = read().unwrap();

	println!("{:?}", x);

	//generate();
}
