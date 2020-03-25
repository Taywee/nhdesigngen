pub mod gtk;
pub mod color;
pub mod design;

use std::error::Error;
use std::io::{BufRead, Seek};
use image::io::Reader;
use image::imageops::FilterType;
use exoquant::{convert_to_indexed, Color, ditherer, optimizer};

pub struct Config<R> where R: BufRead + Seek {
    pub input: Reader<R>,
}

pub fn convert<R>(config: Config<R>) -> Result<(), Box<dyn Error>> where R: BufRead + Seek {
    let input = config.input.decode()?.resize_exact(32, 32, FilterType::Lanczos3).into_rgba();
    let pixels: Vec<Color> = input.pixels().map(|p| Color {
        r: p[0],
        g: p[1],
        b: p[2],
        a: p[3],
    }).collect();
    let (palette, indexed_data) = convert_to_indexed(&pixels, input.width() as usize, 16, &optimizer::KMeans, &ditherer::None);

    println!("P3");
    println!("{} {}", input.width(), input.height());
    println!("255");
    for pixel in indexed_data {
        let color = palette[pixel as usize];
        let hsv: HSVA = color.into();
        let hue: u8 = ((hsv.h / 360.0 * 30.0).floor() as u8).min(29);
        let saturation: u8 = ((hsv.s * 15.0).floor() as u8).min(14);
        let value: u8 = ((hsv.v * 15.0).floor() as u8).min(14);
        let new_color: Color = HSVA {
            h: hue as f32 * (360.0 / 30.0),
            s: saturation as f32 * (1.0 / 15.0),
            v: value as f32 * (1.0 / 15.0),
            a: 1.0,
        }.into();
        //println!("{} {} {}", color.r, color.g, color.b);
        println!("{} {} {}", new_color.r, new_color.g, new_color.b);
    }
    Ok(())
}

