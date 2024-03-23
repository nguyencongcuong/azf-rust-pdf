use std::fs::File;
use std::io::Read;
use std::time::Instant;

use image::GenericImageView;
use zune_core::options::DecoderOptions;
use zune_png::{PngDecoder, PngEncoder};
use zune_png::error::PngDecodeErrors;

pub fn calculate_time<F, R>(operation_name: &str, function: F) -> R where F: FnOnce() -> R {
    let start = Instant::now();
    println!("[{} Start]", operation_name);

    let result = function();

    let elapsed = start.elapsed();
    println!("[{} End] {:?}", operation_name, elapsed);

    return result;
}

fn file_to_bytes(mut file: File) -> Vec<u8> {
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("TODO: panic message");
    buffer
}

pub fn decode_png(path: &str) -> Result<Vec<u8>, PngDecodeErrors> {
    let mut file = File::open(path).unwrap();
    let buffer = file_to_bytes(file);
    let options = DecoderOptions::default().png_set_strip_to_8bit(true);
    let mut decoder = PngDecoder::new(buffer);
    let pixels = decoder.decode_raw();
    pixels
}

pub fn encode_png(raw_pixel: Vec<u8>) -> Vec<u8> {
    let mut encoder = PngEncoder::new(raw_pixel.as_slice(), Default::default());
    let pixels = encoder.encode();

    pixels
}


pub fn open_and_save(path: &str) {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open(path).unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // Write the contents of this image to the Writer in PNG format.
    img.save("test.png").unwrap();
}

pub fn read_image(path: &str) -> printpdf::image_crate::DynamicImage {
    let img = printpdf::image_crate::open(path).unwrap();
    img
}