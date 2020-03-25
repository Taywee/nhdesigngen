pub mod color;
pub mod design;
pub mod gtk;

use image::io::Reader;
use std::io::{BufRead, Seek};

pub struct Config<R>
where
    R: BufRead + Seek,
{
    pub input: Reader<R>,
}
