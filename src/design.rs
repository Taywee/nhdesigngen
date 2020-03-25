use exoquant::optimizer::{KMeans, Optimizer};
use exoquant::{Color, Histogram, Quantizer, SimpleColorSpace};
use image::RgbaImage;
use std::iter::{Extend, IntoIterator};
use std::path::Path;

pub enum Type {
    Simple,
    Pro,
}

pub struct Design {
    palette: Vec<Color>,
    source_image: RgbaImage,
}

impl Default for Design {
    fn default() -> Self {
        Design {
            palette: Vec::new(),
            source_image: RgbaImage::new(32, 32),
        }
    }
}

impl Design {
    pub fn load_palette<F>(&mut self, files: F, type_: Type) -> image::error::ImageResult<()>
    where
        F: IntoIterator,
        F::Item: AsRef<Path>,
    {
        let mut histogram = Histogram::new();
        for path in files {
            let input = image::open(path)?.into_rgba();
            histogram.extend(input.pixels().map(|p| {
                if p[3] == 0 {
                    // Ensure all transparent pixels have the same color
                    Color::new(0, 0, 0, 0)
                } else {
                    Color {
                        r: p[0],
                        g: p[1],
                        b: p[2],
                        a: p[3],
                    }
                }
            }));
        }

        let colorspace = SimpleColorSpace::default();
        let optimizer = KMeans;
        let mut quantizer = Quantizer::new(&histogram, &colorspace);
        while quantizer.num_colors() < 256 {
            quantizer.step();
            // very optional optimization, !very slow!
            // you probably only want to do this every N steps, if at all.
            if quantizer.num_colors() % 64 == 0 {
                quantizer = quantizer.optimize(&optimizer, 4);
            }
        }

        let palette = quantizer.colors(&colorspace);
        self.palette = match type_ {
            // If simple, a transparent color is mandatory
            Type::Simple => {
                let mut new_palette =
                    optimizer.optimize_palette(&colorspace, &palette, &histogram, 16);
                let transparent_index = palette
                    .iter()
                    .enumerate()
                    .find(|(i, c)| c.a == 0)
                    .map(|(i, _)| i);
                match transparent_index {
                    Some(index) => {
                        // Move the transparent color to the end
                        if index != 15 {
                            new_palette.swap(index, 15);
                        }
                        new_palette
                    }
                    None => {
                        // Add a transparent color, even if it's not used.
                        let mut new_palette =
                            optimizer.optimize_palette(&colorspace, &palette, &histogram, 15);
                        new_palette.push(Color {
                            r: 0,
                            g: 0,
                            b: 0,
                            a: 0,
                        });
                        new_palette
                    }
                }
            }
            // No transparency to worry about.  Trust the user to not supply transparent pixels.
            Type::Pro => optimizer.optimize_palette(&colorspace, &palette, &histogram, 15),
        };
        Ok(())
    }
}
