use std::fs::File;
use std::io::Read;
use std::time::Instant;

use image::GenericImageView;

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