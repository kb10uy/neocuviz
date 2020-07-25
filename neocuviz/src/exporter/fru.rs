use super::{
    svg::{SvgElement, SvgEmitter},
    Exporter,
};
use crate::cube::{Cube, CubeFace};

use std::{
    collections::HashMap,
    f64::consts::FRAC_PI_6,
    io::{prelude::*, Result as IoResult},
};

/// F, R, U 面が表示される `Exporter`。
pub struct Fru {
    size: f64,
    colors: HashMap<CubeFace, String>,
}

impl Fru {
    pub fn new(size: f64) -> Fru {
        let mut colors = HashMap::new();
        colors.insert(CubeFace::Front, "#3f0".into());
        colors.insert(CubeFace::Back, "#03c".into());
        colors.insert(CubeFace::Left, "#f90".into());
        colors.insert(CubeFace::Right, "#f30".into());
        colors.insert(CubeFace::Up, "#fff".into());
        colors.insert(CubeFace::Down, "#ff0".into());

        Fru { size, colors }
    }

    fn draw_frame(&self, emitter: &mut SvgEmitter, cube: &Cube) -> IoResult<()> {
        // 外枠
        let points = (0..6)
            .map(|i| {
                let angle = FRAC_PI_6 * (i * 2 + 1) as f64;
                (0.8 * angle.cos(), 0.8 * angle.sin())
            })
            .collect();
        emitter.add_element(SvgElement::StrokePolygon {
            color: "#000".into(),
            thickness: 0.02,
            points,
        });

        // 稜線
        for i in 0..3 {
            let angle = FRAC_PI_6 * (i * 4 + 1) as f64;
            emitter.add_element(SvgElement::Line {
                color: "#000".into(),
                thickness: 0.02,
                start: (0.0, 0.0),
                end: (0.8 * angle.cos(), 0.8 * angle.sin()),
            });
        }

        // エッジライン
        let part_length = 0.8 / cube.divisions() as f64;

        // FU ペア
        let upper = (0.8 * FRAC_PI_6.cos(), 0.8 * FRAC_PI_6.sin());
        let lower = (0.0f64, -0.8);
        for i in 0..(cube.divisions() - 1) {
            let base_length = part_length * (i + 1) as f64;
            let base_point = (
                base_length * (FRAC_PI_6 * 5.0).cos(),
                base_length * (FRAC_PI_6 * 5.0).sin(),
            );
            let points = vec![
                (upper.0 + base_point.0, upper.1 + base_point.1),
                base_point,
                (lower.0 + base_point.0, lower.1 + base_point.1),
            ]
            .into_boxed_slice();
            emitter.add_element(SvgElement::Polyline {
                color: "#000".into(),
                thickness: 0.02,
                points,
            })
        }

        // UR ペア
        let upper = (0.8 * (FRAC_PI_6 * 5.0).cos(), 0.8 * (FRAC_PI_6 * 5.0).sin());
        let lower = (0.0f64, -0.8);
        for i in 0..(cube.divisions() - 1) {
            let base_length = part_length * (i + 1) as f64;
            let base_point = (base_length * FRAC_PI_6.cos(), base_length * FRAC_PI_6.sin());
            let points = vec![
                (upper.0 + base_point.0, upper.1 + base_point.1),
                base_point,
                (lower.0 + base_point.0, lower.1 + base_point.1),
            ]
            .into_boxed_slice();
            emitter.add_element(SvgElement::Polyline {
                color: "#000".into(),
                thickness: 0.02,
                points,
            })
        }

        // FR ペア
        let left = (0.8 * (FRAC_PI_6 * 5.0).cos(), 0.8 * (FRAC_PI_6 * 5.0).sin());
        let right = (0.8 * FRAC_PI_6.cos(), 0.8 * FRAC_PI_6.sin());
        for i in 0..(cube.divisions() - 1) {
            let base_length = part_length * (i + 1) as f64;
            let base_point = (0.0, -base_length);
            let points = vec![
                (left.0 + base_point.0, left.1 + base_point.1),
                base_point,
                (right.0 + base_point.0, right.1 + base_point.1),
            ]
            .into_boxed_slice();
            emitter.add_element(SvgElement::Polyline {
                color: "#000".into(),
                thickness: 0.02,
                points,
            })
        }

        Ok(())
    }

    fn draw_faces(&self, emitter: &mut SvgEmitter, cube: &Cube) -> IoResult<()> {
        let part_length = 0.8 / cube.divisions() as f64;
        let left_diff = (
            part_length * (FRAC_PI_6 * 7.0).cos(),
            part_length * (FRAC_PI_6 * 7.0).sin(),
        );
        let right_diff = (
            part_length * (FRAC_PI_6 * 11.0).cos(),
            part_length * (FRAC_PI_6 * 11.0).sin(),
        );
        let down_diff = (0.0, -part_length);
        let faces = cube.faces();

        // U 面
        for i in 0..9 {
            let color = &self.colors[&faces[&CubeFace::Up][i]];
            let (x, y) = (i % 3, i / 3);
            let base = (
                right_diff.0 * x as f64 + left_diff.0 * y as f64,
                right_diff.1 * x as f64 + left_diff.1 * y as f64 + 0.8,
            );

            let points = vec![
                base,
                (base.0 + right_diff.0, base.1 + right_diff.1),
                (base.0 + down_diff.0, base.1 + down_diff.1),
                (base.0 + left_diff.0, base.1 + left_diff.1),
            ]
            .into_boxed_slice();
            emitter.add_element(SvgElement::FillPolygon {
                color: color.to_owned(),
                points,
            });
        }

        // F 面
        for i in 0..9 {
            let color = &self.colors[&faces[&CubeFace::Front][i]];
            let (x, y) = (i % 3, i / 3);
            let base = (
                right_diff.0 * x as f64 + down_diff.0 * y as f64 + 0.8 * (FRAC_PI_6 * 5.0).cos(),
                right_diff.1 * x as f64 + down_diff.1 * y as f64 + 0.8 * (FRAC_PI_6 * 5.0).sin(),
            );

            let points = vec![
                base,
                (base.0 + right_diff.0, base.1 + right_diff.1),
                (
                    base.0 + down_diff.0 + right_diff.0,
                    base.1 + down_diff.1 + right_diff.1,
                ),
                (base.0 + down_diff.0, base.1 + down_diff.1),
            ]
            .into_boxed_slice();
            emitter.add_element(SvgElement::FillPolygon {
                color: color.to_owned(),
                points,
            });
        }

        // R 面
        for i in 0..9 {
            let color = &self.colors[&faces[&CubeFace::Right][i]];
            let (x, y) = (i % 3, i / 3);
            let base = (
                -left_diff.0 * x as f64 + down_diff.0 * y as f64,
                -left_diff.1 * x as f64 + down_diff.1 * y as f64,
            );

            let points = vec![
                base,
                (base.0 - left_diff.0, base.1 - left_diff.1),
                (base.0 + right_diff.0, base.1 + right_diff.1),
                (base.0 + down_diff.0, base.1 + down_diff.1),
            ]
            .into_boxed_slice();
            emitter.add_element(SvgElement::FillPolygon {
                color: color.to_owned(),
                points,
            });
        }

        Ok(())
    }
}

impl Exporter for Fru {
    fn write<W: Write>(&self, cube: &Cube, mut writer: W) -> IoResult<()> {
        let mut emitter = SvgEmitter::new(self.size, self.size);
        self.draw_faces(&mut emitter, cube)?;
        self.draw_frame(&mut emitter, cube)?;
        emitter.emit(&mut writer)
    }
}
