extern crate image;
use image::{DynamicImage, GenericImage, GenericImageView};

//Struct for storing arguments
pub struct Options {
    pub width: u32,
    pub height: u32,
    pub images: Vec<String>,
}

impl Options {
    pub fn new(width: u32, height: u32, images: Vec<String>) -> Options {
        Options {
            width,
            height,
            images,
        }
    }
}

fn resize_bars(image: DynamicImage, width: u32, height: u32, filter: image::imageops::FilterType) -> image::DynamicImage{
    let image = image.resize(width,height,filter);
    let mut resized_image = DynamicImage::new_rgb8(width,height);
    if image.width() == width {
        resized_image.copy_from(&image, 0, (height-image.height())/2).expect("Failed to copy");
    }else{
        resized_image.copy_from(&image, (width-image.width())/2, 0).expect("Failed to copy");
    }

    resized_image
}

pub fn resize_from_filename(filename: String, width: u32, height: u32) {
    let image = image::open(&filename).expect("Failed read"); //TODO Actually handle errors
    let image = resize_bars(image, width, height, image::imageops::FilterType::Lanczos3);
    image
        .save(format!("{}x{}-{}", width, height, filename))
        .expect("Failed to write");
}
