use super::{
    svg::{SvgElement, SvgEmitter},
    Exporter, ExporterParameters,
};
use crate::cube::{Cube, CubeFace};

use std::{
    collections::HashMap,
    io::{prelude::*, Result as IoResult},
};

/// 上面とその周囲の色を表示する `Exporter`。
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TopLayer {
    size: f64,
    colors: HashMap<CubeFace, String>,
}

impl Exporter for TopLayer {
    fn set_params(&mut self, params: &ExporterParameters) {
        self.colors = params.colors.clone();
        self.size = params.size;
    }

    fn write(&self, cube: &Cube, writer: &mut dyn Write) -> IoResult<()> {
        let mut emitter = SvgEmitter::new(self.size, self.size);
        self.draw(&mut emitter, cube)?;
        emitter.emit(writer)
    }
}

impl TopLayer {
    fn draw(&self, emitter: &mut SvgEmitter, cube: &Cube) -> IoResult<()> {
        let faces = cube.faces();
        let top = &faces[&CubeFace::Up];
        for i in 0..(cube.divisions().pow(2)) {
            let (x, y) = (i % 3, i / 3);
            let color = &self.colors[&top[i]];
            let half = (cube.divisions() as f64 - 1.0) / 2.0;
            let points = vec![
                ((x as f64 - half - 0.5) * 0.32, (half - y as f64 + 0.5) * 0.32),
                ((x as f64 - half + 0.5) * 0.32, (half - y as f64 + 0.5) * 0.32),
                ((x as f64 - half + 0.5) * 0.32, (half - y as f64 - 0.5) * 0.32),
                ((x as f64 - half - 0.5) * 0.32, (half - y as f64 - 0.5) * 0.32),
            ]
            .into_boxed_slice();

            emitter.add_element(SvgElement::StrokeFillPolygon {
                stroke_color: "#000".into(),
                fill_color: color.to_owned(),
                thickness: 0.02,
                points,
            })
        }

        Ok(())
    }
}
