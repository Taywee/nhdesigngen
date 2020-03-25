use exoquant::Color;

struct HSVA {
    pub h: f32,
    pub s: f32,
    pub v: f32,
    pub a: f32,
}

impl From<Color> for HSVA {
    fn from(color: Color) -> Self {
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
        let saturation = if c_max == 0.0 { 0.0 } else { delta / c_max };
        HSVA {
            h: hue,
            s: saturation,
            v: c_max,
            a,
        }
    }
}

impl From<HSVA> for Color {
    fn from(hsv: HSVA) -> Self {
        let a = (hsv.a * 255.0).round() as u8;
        if hsv.v == 0.0 {
            return Color {
                r: 0,
                g: 0,
                b: 0,
                a,
            };
        } else if hsv.s == 0.0 {
            let v = (hsv.v * 255.0).round() as u8;
            return Color {
                r: v,
                g: v,
                b: v,
                a,
            };
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