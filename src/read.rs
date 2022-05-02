pub mod reader {
	use std::path::Path;
	use image;
	use std::error::Error;
	use rqrr::PreparedImage;

	type BoxResult<T> = Result<T, Box<dyn Error>>;
	pub fn scan(filepath: &str) -> BoxResult<Vec<String>> {
		let img = image::open(filepath)?.to_luma8();
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
}