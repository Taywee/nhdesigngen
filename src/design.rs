use crate::color::NHPaletteItem;
use exoquant::ditherer::FloydSteinberg;
use exoquant::optimizer::{KMeans, Optimizer};
use exoquant::{Color, Histogram, Quantizer, SimpleColorSpace, Remapper};
use image::RgbaImage;
use std::iter::{Extend, IntoIterator, repeat_with};
use std::path::Path;

pub struct Design {
    palette: Vec<NHPaletteItem>,
    source_image: RgbaImage,
}

impl Default for Design {
    fn default() -> Self {
        let mut palette: Vec<NHPaletteItem> = repeat_with(|| NHPaletteItem::default()).take(15).collect();
        palette.push(NHPaletteItem::Transparent);
        Design {
            palette,
            source_image: RgbaImage::new(32, 32),
        }
    }
}

impl Design {
    pub fn palette(&self) -> &[NHPaletteItem] {
        &self.palette
    }

    /// Load some files into a contained palette
    pub fn load_palette<F>(&mut self, files: F) -> image::error::ImageResult<()>
    where
        F: IntoIterator,
        F::Item: AsRef<Path>,
    {
        let mut histogram = Histogram::new();
        for path in files {
            let input = image::open(path)?.into_rgba();
            histogram.extend(input.pixels()
                // Filter out all transparent pixels for the purpose of palette generation
                .filter(|p| p[3] > 0)
                .map(|p| {
                    Color {
                        r: p[0],
                        g: p[1],
                        b: p[2],
                        a: p[3],
                    }
            }));
        }

        let colorspace = SimpleColorSpace::default();
        let optimizer = KMeans;
        let mut quantizer = Quantizer::new(&histogram, &colorspace);
        while quantizer.num_colors() < 15 {
            quantizer.step();
            // Maybe remove this, is very slow
            quantizer = quantizer.optimize(&optimizer, 4);
        }

        let palette = quantizer.colors(&colorspace);
        let palette = optimizer.optimize_palette(&colorspace, &palette, &histogram, 16);

        // Convert palette into possible AC colors
        self.palette = palette.into_iter().map(Into::into).collect();
        self.palette.push(NHPaletteItem::Transparent);
        Ok(())
    }

    /// Load an image into the internal image buffer
    pub fn load_image<P>(&mut self, path: P) -> image::error::ImageResult<()>
    where
        P: AsRef<Path>,
    {
        self.source_image = image::open(path)?.into_rgba();
        Ok(())
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.source_image.dimensions()
    }

    /// Generate the indexed design in question 
    pub fn generate(&self) -> Vec<u8> {
        //let ditherer = FloydSteinberg::new();
        let ditherer = exoquant::ditherer::None;
        let colorspace = SimpleColorSpace::default();
        let palette: Vec<Color> = self.palette.iter().map(Into::into).collect();
        let remapper = Remapper::new(&palette, &colorspace, &ditherer);
        let pixels: Vec<Color> = self.source_image.pixels().map(|p| {
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
            }).collect();
        remapper.remap(&pixels, self.source_image.width() as usize)
    }
}
