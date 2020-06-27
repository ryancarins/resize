use image::GenericImageView;
use std::env;
use argparse::{ArgumentParser, Store, List};

fn resize_from_filename(filename: String, width: u32, height: u32) {
    let image = image::open(&filename).expect("Failed read"); //TODO Actually handle errors
    let image = image.resize_to_fill(width,height,image::imageops::FilterType::Nearest);
    image.save(format!("1366{}",filename)).expect("Failed to write");
}

fn main() {
    //Variables for parsing
    let mut width = 1366;
    let mut height = 768;
    let mut images: Vec<String> = Vec::new();
    {
    let mut parser = argparse::ArgumentParser::new();
    parser.set_description("Bulk image resizer");
    parser.refer(&mut width).add_option(&["-W","--width"], Store, "Set width (default 1366)");
    parser.refer(&mut height).add_option(&["-H","--height"], Store, "Set height (default 768)");
    parser.refer(&mut images).add_option(&["-i","--images"], List, "Set image list");

    parser.parse_args_or_exit();
    }


    //let args: Vec<String> = env::args().collect();
    for image in images {
        resize_from_filename(image,width,height);
    }
}
