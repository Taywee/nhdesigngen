use crate::color::NHPaletteItem;
use exoquant::optimizer::Optimizer;
use exoquant::ditherer::Ditherer;
use exoquant::{Color, Histogram, Quantizer, SimpleColorSpace, Remapper};
use std::iter::{Extend, IntoIterator, repeat_with};

pub struct Design {
    source_image: Vec<Color>,
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
            source_image: vec![Color::new(0, 0, 0, 255)],
            dimensions: (1, 1),
        }
    }
}

impl Design {
    pub fn palette(&self) -> &[NHPaletteItem] {
        &self.palette
    }

    /// Load some files into a contained histogram
    pub fn load_histogram_from_buffers(&mut self, files: Vec<Vec<(u8, u8, u8, u8)>>) {
        //TODO: Add >64x64 per-image error condition
        self.histogram = Histogram::new();
        for file in files {
            self.histogram.extend(file.iter()
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
        where D: AsRef<dyn Ditherer>
    {
        let ditherer = ditherer.as_ref();
        let colorspace = SimpleColorSpace::default();
        let palette: Vec<Color> = self.palette.iter().map(Into::into).collect();
        let remapper = Remapper::new(&palette, &colorspace, ditherer);
        let pixels: Vec<Color> = self.source_image.iter().map(|p| {
                if p.a == 0 {
                    // Ensure all transparent pixels have the same color
                    Color::new(0, 0, 0, 0)
                } else {
                    p.clone()
                }
            }).collect();
        remapper.remap(&pixels, self.dimensions.0)
    }
}
