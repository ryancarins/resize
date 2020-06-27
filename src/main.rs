extern crate image;
use argparse::{ArgumentParser, Store, Collect};


//Struct for storing arguments
struct Options {
    width: u32,
    height: u32,
    images: Vec<String>,
}

impl Options {
    fn new(width: u32, height: u32, images: Vec<String>) -> Options {
        Options{width, height, images}
    }
}

fn resize_from_filename(filename: String, width: u32, height: u32) {
    let image = image::open(&filename).expect("Failed read"); //TODO Actually handle errors
    let image = image.resize_to_fill(width,height,image::imageops::FilterType::Lanczos3);
    image.save(format!("1366{}",filename)).expect("Failed to write");
}

fn main() {
    let mut options = Options::new(1366, 768, Vec::new());

    {
    let mut parser = ArgumentParser::new();
    parser.set_description("Bulk image resizer");
    parser.refer(&mut options.width)
        .add_option(&["-W","--width"], Store, "Set width (default 1366)");
    parser.refer(&mut options.height)
        .add_option(&["-H","--height"], Store, "Set height (default 768)");
    parser.refer(&mut options.images)
        .add_argument("--images", Collect, "Set image list");

    parser.parse_args_or_exit();
    }

    for image in options.images {
        resize_from_filename(image,options.width,options.height);
    }
}
