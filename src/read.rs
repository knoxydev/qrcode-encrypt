pub mod reader {
	use std::path::Path;
	use image;
	use std::error::Error;
	use rqrr::PreparedImage;

	pub fn scan(filepath: &str, encode: &str) -> Result<String, Box<dyn Error>> {
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
		
		if encode == "base64" {
			let x = base64::decode(contents[0].to_string()).unwrap();
			res = String::from_utf8_lossy(&x).to_string();
		}
		else if encode == "hex" { 
			let x = hex::decode(contents[0].to_string());
			res = String::from_utf8_lossy(&x.unwrap()).to_string();
		}
		else if encode == "txt" { res = contents[0].to_string(); }
		else { println!("Incorrect encode method"); }

		Ok(res)
	}
}