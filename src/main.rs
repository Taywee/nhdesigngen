use clap::{Arg, App};
use image::io::Reader;
use nhdesigngen::Config;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = App::new("New Horizons design generator")
        .version("0.1.0")
        .author("Taylor C. Richberger <https://gitlab.com/Taywee>")
        .about("Creates Animal Crossing New Horizons designs from image input")
        .arg(Arg::with_name("input")
            .value_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    let filename = args.value_of("input").unwrap();
    let input = Reader::open(filename)?;

    nhdesigngen::convert(Config{
        input,
    })
}
