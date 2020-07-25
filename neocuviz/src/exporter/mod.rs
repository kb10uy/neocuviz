mod fru;
pub use fru::Fru;

use crate::cube::Cube;
use std::io::{prelude::*, Result as IoResult};

/// SVG を出力する構造体が実装するべきトレイト。
pub trait Exporter {
    fn write<W: Write>(&self, cube: &Cube, writer: W) -> IoResult<()>;
}
