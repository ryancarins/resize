extern crate image;
use image::{DynamicImage, GenericImage, GenericImageView};
use std::process;

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

fn resize_bars(
    image: DynamicImage,
    width: u32,
    height: u32,
    filter: image::imageops::FilterType,
) -> image::DynamicImage {
    let image = image.resize(width, height, filter);

    if width as f32 / height as f32 == image.width() as f32 / image.height() as f32 {
        //Same aspect ratio
        return image;
    }

    let mut resized_image = match image.color() {
        image::ColorType::Rgb8 => DynamicImage::new_rgb8(width, height),
        image::ColorType::Rgb16 => DynamicImage::new_rgb16(width, height),
        image::ColorType::Rgba8 => DynamicImage::new_rgba8(width, height),
        image::ColorType::Rgba16 => DynamicImage::new_rgba16(width, height),
        image::ColorType::Bgr8 => DynamicImage::new_bgr8(width, height),
        image::ColorType::Bgra8 => DynamicImage::new_bgra8(width, height),
        image::ColorType::L8 => DynamicImage::new_luma8(width, height),
        image::ColorType::La8 => DynamicImage::new_luma8(width, height),
        image::ColorType::L16 => DynamicImage::new_luma16(width, height),
        image::ColorType::La16 => DynamicImage::new_luma16(width, height),
        _ => {
            eprintln!("Image type error");
            process::exit(1);
        }
    };

    if image.width() == width {
        resized_image
            .copy_from(&image, 0, (height - image.height()) / 2)
            .expect("Failed to copy");
    } else {
        resized_image
            .copy_from(&image, (width - image.width()) / 2, 0)
            .expect("Failed to copy");
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
