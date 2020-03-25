pub mod window;
mod error;

pub use window::Window;
pub use error::Error;

use glib::object::IsA;
use glib::object::Object;
use gtk::prelude::*;
use std::rc::{Rc, Weak};

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
