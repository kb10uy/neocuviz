mod fru;
mod svg;
mod top_layer;

pub use fru::Fru;
pub use top_layer::TopLayer;

use crate::cube::{Cube, CubeFace};
use std::{collections::HashMap, io::{prelude::*, Result as IoResult}};

/// Exporter に提供される共通パラメーター
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ExporterParameters {
    pub colors: HashMap<CubeFace, String>,
    pub size: f64,
}

/// SVG を出力する構造体が実装するべきトレイト。
pub trait Exporter {
    /// 共通パラメーターを設定する。
    fn set_params(&mut self, params: &ExporterParameters);

    /// SVG を書き出す。
    fn write(&self, cube: &Cube, writer: &mut dyn Write) -> IoResult<()>;
}
