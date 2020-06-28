use argparse::{ArgumentParser, Collect, Store};
use resize::Options;
use std::process;

fn main() {
    let default_width = 1366;
    let default_height = 768;
    let mut options = Options::new(default_width, default_height, Vec::new());

    //Handle command line arguments
    {
        //Using variables here because I wanted to format and parser take a &str
        let height_text = format!("Set height (default {})", default_height);
        let width_text = format!("Set width (default {})", default_width);

        let mut parser = ArgumentParser::new();
        parser.set_description("Bulk image resizer");
        parser
            .refer(&mut options.width)
            .add_option(&["-W", "--width"], Store, &width_text);

        parser
            .refer(&mut options.height)
            .add_option(&["-H", "--height"], Store, &height_text);

        parser
            .refer(&mut options.images)
            .add_argument("--images", Collect, "Set image list")
            .required();

        parser.parse_args_or_exit();
    }

    if options.images.len() < 1 {
        eprintln!("At least on file required for resizing");
        process::exit(1);
    }

    for image in options.images {
        resize::resize_from_filename(image, options.width, options.height);
    }
}
