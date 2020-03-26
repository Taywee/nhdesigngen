use crate::color::NHPaletteItem;
use exoquant::optimizer::Optimizer;
use exoquant::ditherer::Ditherer;
use exoquant::{Color, Histogram, Quantizer, SimpleColorSpace, Remapper};
use std::iter::{Extend, IntoIterator, repeat_with};

pub struct Design {
    image: Vec<Color>,
    dimensions: (usize, usize),
    histogram: Histogram,
    palette: Vec<NHPaletteItem>,
}

impl Default for Design {
    fn default() -> Self {
        let mut palette: Vec<NHPaletteItem> = repeat_with(|| NHPaletteItem::default()).take(15).collect();
        palette.push(NHPaletteItem::Transparent);
        Design {
            palette,
            histogram: Histogram::new(),
            image: vec![Color::new(0, 0, 0, 255)],
            dimensions: (1, 1),
        }
    }
}

impl Design {
    pub fn palette(&self) -> &[NHPaletteItem] {
        &self.palette
    }

    /// Load the full image
    pub fn load_image(&mut self, image: Vec<(u8, u8, u8, u8)>, dimensions: (usize, usize)) -> Result<(), String> {
        if image.len() != dimensions.0 * dimensions.1 {
            return Err(format!("There was an error with the image dimensions.  The dimensions say the image should be {}x{} for {} pixels, but the image was actually {} pixels", dimensions.0, dimensions.1, dimensions.0 * dimensions.1, image.len()));
        }
        if dimensions.0 > 64 || dimensions.1 > 64 {
            return Err(format!("The max image size is 64x64, but you tried to load an image with {}x{} pixels", dimensions.0, dimensions.1));
        }
        self.image = image.into_iter().map(|pixel| {
            if pixel.3 == 0 {
                Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                }
            } else {
                Color {
                    r: pixel.0,
                    g: pixel.1,
                    b: pixel.2,
                    a: pixel.3,
                }
            }
        }).collect();
        self.dimensions = dimensions;
        Ok(())
    }

    /// Load some files into a contained histogram
    pub fn load_histogram(&mut self, images: Vec<Vec<(u8, u8, u8, u8)>>) -> Result<(), String> {
        self.histogram = Histogram::new();
        for image in images {
            if image.len() > 4096 {
                return Err(format!("The max allowed image size is 4096 pixels (64x64).  You tried to load an image with {} pixels", image.len()));
            }
            self.histogram.extend(image.iter()
                // Filter out all transparent pixels for the purpose of palette generation
                .filter(|p| p.3 > 0)
                .map(|p| {
                    //crate::log("Making color");
                    Color {
                        r: p.0,
                        g: p.1,
                        b: p.2,
                        a: p.3,
                    }
            }));
        }
        Ok(())
    }

    /// Load some files into a contained palette
    pub fn optimize_palette<O>(&mut self, optimizer: O)
    where
        O: Optimizer,
    {
        let colorspace = SimpleColorSpace::default();
        let mut quantizer = Quantizer::new(&self.histogram, &colorspace);
        while quantizer.num_colors() < 15 {
            quantizer.step();
            // Maybe remove this, is very slow
            quantizer = quantizer.optimize(&optimizer, 4);
        }

        let palette = quantizer.colors(&colorspace);
        let palette = optimizer.optimize_palette(&colorspace, &palette, &self.histogram, 16);

        // Convert palette into possible AC colors
        self.palette = palette.into_iter().map(Into::into).collect();
        self.palette.push(NHPaletteItem::Transparent);
    }

    pub fn dimensions(&self) -> (usize, usize) {
        self.dimensions
    }

    /// Generate the indexed design in question 
    pub fn generate<D>(&self, ditherer: D) -> Vec<u8>
        where D: Ditherer
    {
        let colorspace = SimpleColorSpace::default();
        let palette: Vec<Color> = self.palette.iter().map(Into::into).collect();
        let remapper = Remapper::new(&palette, &colorspace, &ditherer);
        remapper.remap(&self.image, self.dimensions.0)
    }
}
