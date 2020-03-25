mod error;
pub mod window;

pub use error::Error;
pub use window::Window;

use glib::object::IsA;
use glib::object::Object;
use gtk::prelude::*;
use crate::color::NHPaletteItem;

type Result<T> = std::result::Result<T, Error>;

pub trait TryGet {
    fn try_get<R>(&self, label: &str) -> Result<R>
    where
        R: IsA<Object>;
}

impl TryGet for gtk::Builder {
    fn try_get<R>(&self, label: &str) -> Result<R>
    where
        R: IsA<Object>,
    {
        let object = self
            .get_object(label)
            .ok_or_else(|| Error::ObjectNotFound {
                label: label.into(),
            })?;
        Ok(object)
    }
}

// Extra traits

impl From<&NHPaletteItem> for gdk::RGBA {
    fn from(palette_item: &NHPaletteItem) -> gdk::RGBA {
        let color: exoquant::Color = palette_item.into();
        gdk::RGBA {
            red: color.r as f64 / 255.0,
            green: color.g as f64 / 255.0,
            blue: color.b as f64 / 255.0,
            alpha: color.a as f64 / 255.0,
        }
    }
}
