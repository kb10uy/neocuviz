use super::Exporter;
use crate::cube::{Cube, CubeFace};
use std::{
    collections::HashMap,
    f64::consts::FRAC_PI_6,
    io::{prelude::*, Result as IoResult},
};

/// F, R, U 面が表示される `Exporter`。
pub struct Fru {
    size: f64,
    stroke_width: f64,
}

impl Fru {
    pub fn new(size: f64) -> Fru {
        Fru {
            size,
            stroke_width: size * 0.02,
        }
    }

    fn draw_frame<W: Write>(&self, mut writer: W, cube: &Cube) -> IoResult<()> {
        let line_length = self.size * 0.8;
        let mut buffer = String::with_capacity(1024);

        // 外枠
        for i in 0..6 {
            let angle = FRAC_PI_6 * (i * 2 + 1) as f64;
            buffer.push_str(&format!(
                "{:.5} {:.5}, ",
                self.size + line_length * angle.cos(),
                self.size + -(line_length * angle.sin())
            ));
        }
        writeln!(
            writer,
            r#"  <polygon fill="transparent" stroke="black" stroke-width="{}" points="{}"/>"#,
            self.stroke_width, buffer
        )?;

        // 稜線
        buffer.clear();
        for i in 0..3 {
            let angle = FRAC_PI_6 * (i * 4 + 1) as f64;
            writeln!(
                writer,
                r#"  <line stroke="black" stroke-width="{}" x1="{:.5}" y1="{:.5}" x2="{:.5}" y2="{:.5}"/>"#,
                self.stroke_width,
                self.size,
                self.size,
                self.size + line_length * angle.cos(),
                self.size + -(line_length * angle.sin())
            )?;
        }

        // エッジライン
        let part_length = line_length / cube.divisions() as f64;

        // FU ペア
        let upper = (line_length * FRAC_PI_6.cos(), line_length * FRAC_PI_6.sin());
        let lower = (0.0f64, -line_length);
        for i in 0..(cube.divisions() - 1) {
            let base_length = part_length * (i + 1) as f64;
            let base_point = (
                base_length * (FRAC_PI_6 * 5.0).cos(),
                base_length * (FRAC_PI_6 * 5.0).sin(),
            );
            writeln!(
                writer,
                r#"  <polyline fill="transparent" stroke="black" stroke-width="{}" points="{:.5} {:.5}, {:.5} {:.5}, {:.5} {:.5}"/>"#,
                self.stroke_width,
                self.size + (upper.0 + base_point.0),
                self.size + -(upper.1 + base_point.1),
                self.size + base_point.0,
                self.size + -base_point.1,
                self.size + (lower.0 + base_point.0),
                self.size + -(lower.1 + base_point.1),
            )?;
        }

        // UR ペア
        let upper = (
            line_length * (FRAC_PI_6 * 5.0).cos(),
            line_length * (FRAC_PI_6 * 5.0).sin(),
        );
        let lower = (0.0f64, -line_length);
        for i in 0..(cube.divisions() - 1) {
            let base_length = part_length * (i + 1) as f64;
            let base_point = (base_length * FRAC_PI_6.cos(), base_length * FRAC_PI_6.sin());
            writeln!(
                writer,
                r#"  <polyline fill="transparent" stroke="black" stroke-width="{}" points="{:.5} {:.5}, {:.5} {:.5}, {:.5} {:.5}"/>"#,
                self.stroke_width,
                self.size + (upper.0 + base_point.0),
                self.size + -(upper.1 + base_point.1),
                self.size + base_point.0,
                self.size + -base_point.1,
                self.size + (lower.0 + base_point.0),
                self.size + -(lower.1 + base_point.1),
            )?;
        }

        // FR ペア
        let left = (
            line_length * (FRAC_PI_6 * 5.0).cos(),
            line_length * (FRAC_PI_6 * 5.0).sin(),
        );
        let right = (line_length * FRAC_PI_6.cos(), line_length * FRAC_PI_6.sin());
        for i in 0..(cube.divisions() - 1) {
            let base_length = part_length * (i + 1) as f64;
            let base_point = (0.0, -base_length);
            writeln!(
                writer,
                r#"  <polyline fill="transparent" stroke="black" stroke-width="{}" points="{:.5} {:.5}, {:.5} {:.5}, {:.5} {:.5}"/>"#,
                self.stroke_width,
                self.size + (left.0 + base_point.0),
                self.size + -(left.1 + base_point.1),
                self.size + base_point.0,
                self.size + -base_point.1,
                self.size + (right.0 + base_point.0),
                self.size + -(right.1 + base_point.1),
            )?;
        }

        Ok(())
    }

    fn draw_faces<W: Write>(&self, mut writer: W, cube: &Cube) -> IoResult<()> {
        let line_length = self.size * 0.8;
        let part_length = line_length / cube.divisions() as f64;
        let left_diff = (
            part_length * (FRAC_PI_6 * 7.0).cos(),
            part_length * (FRAC_PI_6 * 7.0).sin(),
        );
        let right_diff = (
            part_length * (FRAC_PI_6 * 11.0).cos(),
            part_length * (FRAC_PI_6 * 11.0).sin(),
        );
        let down_diff = (0.0, -part_length);

        let mut colors = HashMap::new();
        colors.insert(CubeFace::Front, "#3f0");
        colors.insert(CubeFace::Back, "#03c");
        colors.insert(CubeFace::Left, "#f90");
        colors.insert(CubeFace::Right, "#f30");
        colors.insert(CubeFace::Up, "#fff");
        colors.insert(CubeFace::Down, "#fc0");

        let faces = cube.faces();
        eprintln!("{:?}", faces);

        for i in 0..9 {
            let color = colors[&faces[&CubeFace::Up][i]];
            let (x, y) = (i % 3, i / 3);
            let base = (
                right_diff.0 * x as f64 + left_diff.0 * y as f64,
                right_diff.1 * x as f64 + left_diff.1 * y as f64,
            );

            writeln!(
                writer,
                r#"  <polygon fill="{}" points="{} {}, {} {}, {} {}, {} {}"/>"#,
                color,
                self.size + base.0,
                self.size - base.1,
                self.size + base.0 + right_diff.0,
                self.size - base.1 - right_diff.1,
                self.size + base.0 + down_diff.0,
                self.size - base.1 - down_diff.1,
                self.size + base.0 + left_diff.0,
                self.size - base.1 - left_diff.1,
            )?;
        }

        Ok(())
    }
}

impl Exporter for Fru {
    fn write<W: Write>(&self, cube: &Cube, mut writer: W) -> IoResult<()> {
        writeln!(writer, r#"<?xml version="1.0"?>"#)?;
        writeln!(
            writer,
            r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="{:.6}" height="{:.5}">"#,
            self.size * 2.0,
            self.size * 2.0,
        )?;
        self.draw_faces(&mut writer, cube)?;
        self.draw_frame(&mut writer, cube)?;
        writeln!(writer, r#"</svg>"#)?;

        Ok(())
    }
}
