use image::{open, GenericImageView, ImageFormat};
use image::imageops::FilterType;

fn main() {
    
    let file_name = "assets/lena30.jpg";
    let img = open(file_name).expect("Image file should exist");

    let (width, height) = img.dimensions();
    println!("{}: Width: {}, Height: {}",file_name, width, height);

    // Saving a png file
    if let Err(e) = img.save_with_format("assets/lena30.png", ImageFormat::Png) {
        println!("Failed to save image as PNG: {}", e);
    }

    // Saving a webp file
    if let Err(e) = img.save_with_format("assets/lena30.webp", ImageFormat::WebP) {
        println!("Failed to save image as WEBP: {}", e);
    }

    let resized_img = img.resize(128, 128, FilterType::Lanczos3);
    if let Err(e) = resized_img.save_with_format("assets/lena30_RESIZED.png", ImageFormat::Png) {
        println!("Failed to save image as PNG: {}", e);
    }
}
