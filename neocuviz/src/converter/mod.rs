use std::io::{prelude::*, Result as IoResult};

/// SVG から他の画像形式に変換するトレイト。
pub trait Converter {
    fn convert<W: Write>(&self, writer: W, svg_source: &str) -> IoResult<()>;
}
