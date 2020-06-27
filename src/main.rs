use image::GenericImageView;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for image in &args[1..] {
        println!("{:?}",image);
        let testimg = image::open(image).expect("Failed read");
        println!("{:?}",testimg.dimensions());
        let testimg = testimg.resize_to_fill(1366,768,image::imageops::FilterType::Nearest);
        testimg.save(format!("1366{}",image)).expect("Failed to write");
    }
}
