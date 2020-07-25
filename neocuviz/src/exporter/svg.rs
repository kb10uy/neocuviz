//! Exporter で SVG の出力に関係する型のモジュール

use std::io::{prelude::*, Result as IoResult};

/// SVG で描画される要素(最低限)。
#[derive(Debug, Clone, PartialEq)]
pub enum SvgElement {
    /// 単一の線分
    Line {
        color: String,
        thickness: f64,
        start: (f64, f64),
        end: (f64, f64),
    },

    /// 連続かつループしない線分
    Polyline {
        color: String,
        thickness: f64,
        points: Box<[(f64, f64)]>,
    },

    /// 連続かつループする線分
    StrokePolygon {
        color: String,
        thickness: f64,
        points: Box<[(f64, f64)]>,
    },

    /// 塗り潰される多角形
    FillPolygon {
        color: String,
        points: Box<[(f64, f64)]>,
    },
}

/// SVG 出力のヘルパー。
#[derive(Debug, Clone)]
pub struct SvgEmitter {
    /// 出力サイズの幅
    width: f64,

    /// 出力サイズの高さ
    height: f64,

    /// 倍率
    transform_scale: f64,

    /// 描画される SVG 要素
    elements: Vec<SvgElement>,
}

impl SvgEmitter {
    /// 出力サイズのみを指定してインスタンスを生成する。
    /// 象限サイズについては、短辺が正負 1 の範囲が収まるように調整される。
    pub fn new(width: f64, height: f64) -> SvgEmitter {
        let shorter = f64::min(width, height);
        let transform_scale = shorter / 2.0;

        SvgEmitter {
            width,
            height,
            transform_scale,
            elements: vec![],
        }
    }

    /// `SvgElement` を追加する。
    pub fn add_element(&mut self, element: SvgElement) {
        self.elements.push(element);
    }

    /// SVG テキストデータを出力する。
    pub fn emit(&self, mut writer: impl Write) -> IoResult<()> {
        write!(writer, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
        write!(
            writer,
            r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="{:.2}" height="{:.2}">"#,
            self.width, self.height
        )?;
        for element in &self.elements {
            self.emit_element(&mut writer, element)?;
        }
        write!(writer, r#"</svg>"#)?;
        Ok(())
    }

    fn emit_element(&self, mut writer: impl Write, element: &SvgElement) -> IoResult<()> {
        match element {
            SvgElement::Line {
                color,
                thickness,
                start,
                end,
            } => {
                let (sx, sy) = self.transform_point(*start);
                let (ex, ey) = self.transform_point(*end);
                write!(
                    writer,
                    r#"<line stroke-width="{:.5}" stroke="{}" x1="{:.5}" y1="{:.5}" x2="{:.5}" y2="{:.5}"/>"#,
                    thickness * self.transform_scale,
                    color,
                    sx,
                    sy,
                    ex,
                    ey
                )?;
            }
            SvgElement::Polyline {
                color,
                thickness,
                points,
            } => {
                write!(
                    writer,
                    r#"<polyline stroke-width="{:.5}" stroke="{}" fill="none" points=""#,
                    thickness * self.transform_scale,
                    color
                )?;
                for point in points.iter() {
                    let (x, y) = self.transform_point(*point);
                    write!(writer, "{} {},", x, y)?;
                }
                write!(writer, r#""/>"#)?;
            }
            SvgElement::StrokePolygon {
                color,
                thickness,
                points,
            } => {
                write!(
                    writer,
                    r#"<polygon stroke-width="{:.5}" stroke="{}" fill="none" points=""#,
                    thickness * self.transform_scale,
                    color
                )?;
                for point in points.iter() {
                    let (x, y) = self.transform_point(*point);
                    write!(writer, "{:.5} {:.5},", x, y)?;
                }
                write!(writer, r#""/>"#)?;
            }
            SvgElement::FillPolygon { color, points } => {
                write!(writer, r#"<polygon fill="{}" points=""#, color)?;
                for point in points.iter() {
                    let (x, y) = self.transform_point(*point);
                    write!(writer, "{:.5} {:.5},", x, y)?;
                }
                write!(writer, r#""/>"#)?;
            }
        }

        Ok(())
    }

    fn transform_point(&self, (x, y): (f64, f64)) -> (f64, f64) {
        (
            self.width / 2.0 + x * self.transform_scale,
            self.height / 2.0 - y * self.transform_scale,
        )
    }
}
