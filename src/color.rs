use exoquant::Color;

/// Simple full-range HSV+alpha for conversions
#[derive(Debug, Clone)]
pub struct HSVA {
    /// Hue, in degrees [0, 360)
    pub h: f32,
    /// Saturation [0, 1]
    pub s: f32,
    /// Value [0, 1]
    pub v: f32,
    /// Alpha [0, 1]
    pub a: f32,
}



/// Quantized HSV
#[derive(Debug, Default, Clone)]
pub struct NHColor {
    pub h: u8,
    pub s: u8,
    pub v: u8,
}

/// Palette item allowing HSV or transparent
#[derive(Debug, Clone)]
pub enum NHPaletteItem {
    Color(NHColor),
    Transparent,
}

impl Default for HSVA {
    fn default() -> Self {
        HSVA {
            h: 0.0,
            s: 0.0,
            v: 0.0,
            a: 1.0,
        }
    }
}

impl Default for NHPaletteItem {
    fn default() -> Self {
        NHPaletteItem::Color(Default::default())
    }
}

impl From<&Color> for HSVA {
    fn from(color: &Color) -> Self {
        let a = color.a as f32 / 255.0;

        let r_prime = color.r as f32 / 255.0;
        if color.r == color.g && color.g == color.b {
            return HSVA {
                h: 0.0,
                s: 0.0,
                v: r_prime,
                a,
            };
        }
        let g_prime = color.g as f32 / 255.0;
        let b_prime = color.b as f32 / 255.0;
        let c_max = r_prime.max(g_prime).max(b_prime);
        let c_min = r_prime.min(g_prime).min(b_prime);
        let delta = c_max - c_min;
        let mut hue = 60.0
            * if c_max == r_prime {
                (g_prime - b_prime) / delta
            } else if c_max == g_prime {
                (b_prime - r_prime) / delta + 2.0
            } else {
                (r_prime - g_prime) / delta + 4.0
            };
        while hue < 0.0 {
            hue += 360.0;
        }
        while hue >= 360.0 {
            hue -= 360.0;
        }
        let saturation = if c_max == 0.0 { 0.0 } else { delta / c_max };
        HSVA {
            h: hue,
            s: saturation,
            v: c_max,
            a,
        }
    }
}

impl From<&HSVA> for Color {
    fn from(hsv: &HSVA) -> Self {
        let a = (hsv.a * 255.0).round() as u8;
        if hsv.v == 0.0 {
            return Color::new(0, 0, 0, a);
        } else if hsv.s == 0.0 {
            let v = (hsv.v * 255.0).round() as u8;
            return Color::new(v, v, v, a);
        }
        let chroma = hsv.v * hsv.s;
        let h_prime = hsv.h / 60.0;
        let x = chroma * (1.0 - (h_prime % 2.0 - 1.0).abs());
        let (r_prime, g_prime, b_prime) = if h_prime >= 0.0 && h_prime <= 1.0 {
            (chroma, x, 0.0)
        } else if h_prime <= 2.0 {
            (x, chroma, 0.0)
        } else if h_prime <= 3.0 {
            (0.0, chroma, x)
        } else if h_prime <= 4.0 {
            (0.0, x, chroma)
        } else if h_prime <= 5.0 {
            (x, 0.0, chroma)
        } else if h_prime <= 6.0 {
            (chroma, 0.0, x)
        } else {
            panic!("this should never ever happen");
        };
        let m = hsv.v - chroma;
        Color {
            r: ((r_prime + m) * 255.0).round() as u8,
            g: ((g_prime + m) * 255.0).round() as u8,
            b: ((b_prime + m) * 255.0).round() as u8,
            a,
        }
    }
}

impl From<&HSVA> for NHPaletteItem {
    fn from(hsv: &HSVA) -> Self {
        if hsv.a == 0.0 {
            NHPaletteItem::Transparent
        } else {
            NHPaletteItem::Color(NHColor {
                // Hue is [0, 29] for 30 full colors.  30 is full red, and has wraparound, so we scale
                // from 0-30 and set a modulus
                h: ((hsv.h / 360.0 * 30.0).round() as u8) % 30,

                // Saturation is [0, 14], and doesn't actually allow full saturation, so we
                // scale to [0, 15] and have to drop the top.  Later, an option for global
                // desaturation may be made possible, but until then, it will have to be manual.
                s: ((hsv.s * 15.0).round() as u8).min(14).max(0),

                // The value behavior is exactly like the saturation behavior above.
                v: ((hsv.v * 15.0).round() as u8).min(14).max(0),
            })
        }
    }
}

impl From<&NHPaletteItem> for HSVA {
    fn from(palette_item: &NHPaletteItem) -> Self {
        match palette_item {
            NHPaletteItem::Transparent => HSVA {
                h: 0.0,
                s: 0.0,
                v: 0.0,
                a: 0.0,
            },
            NHPaletteItem::Color(color) => HSVA {
                h: color.h as f32 * (360.0 / 30.0),
                s: color.s as f32 / 15.0,
                v: color.v as f32 / 15.0,
                a: 1.0,
            },
        }
    }
}

impl From<&Color> for NHPaletteItem {
    fn from(color: &Color) -> Self {
        let hsv: HSVA = color.into();
        (&hsv).into()
    }
}

impl From<&NHPaletteItem> for Color {
    fn from(palette_item: &NHPaletteItem) -> Self {
        let hsv: HSVA = palette_item.into();
        (&hsv).into()
    }
}

// Convenience conversion traits for moves

impl From<Color> for HSVA {
    fn from(color: Color) -> Self {
        (&color).into()
    }
}

impl From<HSVA> for Color {
    fn from(hsv: HSVA) -> Self {
        (&hsv).into()
    }
}

impl From<HSVA> for NHPaletteItem {
    fn from(hsv: HSVA) -> Self {
        (&hsv).into()
    }
}

impl From<NHPaletteItem> for HSVA {
    fn from(palette_item: NHPaletteItem) -> Self {
        (&palette_item).into()
    }
}

impl From<Color> for NHPaletteItem {
    fn from(color: Color) -> Self {
        (&color).into()
    }
}

impl From<NHPaletteItem> for Color {
    fn from(palette_item: NHPaletteItem) -> Self {
        (&palette_item).into()
    }
}
