use neocuviz::{
    cube::{Cube, CubeFace},
    exporter::{Exporter, ExporterParameters, Fru, TopLayer},
    notation::{Movement, Movements},
};
use std::{
    collections::HashMap,
    fs::File,
    io::{
        prelude::*, stdin, stdout, BufReader, BufWriter, Error as IoError, ErrorKind,
        Result as IoResult,
    },
};

use clap::Clap;
use image::{png::PNGEncoder, ColorType};
use usvg::{FitTo, Options, Tree};

#[clap(version, author)]
#[derive(Clap)]
struct Arguments {
    /// キューブのサイズ(分割数)
    #[clap(short = "s", long, default_value = "3")]
    cube_size: usize,

    /// 出力ファイルの視点(描画方法)を指定する
    #[clap(short = "v", long, default_value = "fru", validator=is_valid_view_type)]
    view_type: String,

    /// 出力ファイルのフォーマットを指定する
    #[clap(short = "f", long, default_value = "svg", validator=is_valid_format)]
    output_format: String,

    /// 出力ファイルの解像度を指定する
    #[clap(short = "r", long, default_value = "512")]
    resolution: usize,

    /// 入力の手順の逆手順を適用する
    #[clap(short = "i", long)]
    invert: bool,

    /// 適用する回転記号列。
    /// 省略された場合は標準入力から読み込む
    movements: Option<String>,

    /// 出力先のファイル名。
    /// 省略された場合は標準出力に書き込む
    output: Option<String>,
}

fn is_valid_format(value: &str) -> Result<(), String> {
    match value {
        "svg" | "png" => Ok(()),
        _ => Err(format!("Invalid output format: {}", value)),
    }
}

fn is_valid_view_type(value: &str) -> Result<(), String> {
    match value {
        "fru" | "toplayer" => Ok(()),
        _ => Err(format!("Invalid view type: {}", value)),
    }
}

fn main() -> IoResult<()> {
    let args = Arguments::parse();

    // 入力と出力
    let (mut stdin_instance, mut stdout_instance);
    let (mut infile, mut outfile);
    let reader: &mut dyn Read = if let Some(filename) = &args.movements {
        infile = BufReader::new(File::open(filename)?);
        &mut infile
    } else {
        stdin_instance = BufReader::new(stdin());
        &mut stdin_instance
    };
    let writer: &mut dyn Write = if let Some(filename) = &args.output {
        outfile = BufWriter::new(File::create(filename)?);
        &mut outfile
    } else {
        stdout_instance = BufWriter::new(stdout());
        &mut stdout_instance
    };

    // キューブ操作
    let mut cube = Cube::new(args.cube_size);
    let mut movements_str = String::with_capacity(1024);
    reader.read_to_string(&mut movements_str)?;

    let mut movements = vec![];
    let movements = Movements::new(&movements_str)
        .try_fold(&mut movements, |movs, m| match m {
            Ok(movement) => {
                movs.push(movement);
                Ok(movs)
            }
            Err(e) => Err(e),
        })
        .map_err(|e| IoError::new(ErrorKind::Other, e))?;

    if args.invert {
        for movement in Movement::inverse_sequence(movements.iter()) {
            cube.apply(movement).unwrap();
        }
    } else {
        for movement in movements {
            cube.apply(*movement).unwrap();
        }
    }


    // 描画
    let mut exporter: Box<dyn Exporter> = match &args.view_type[..] {
        "fru" => Box::new(Fru::default()),
        "toplayer" => Box::new(TopLayer::default()),
        _ => unreachable!(),
    };
    let params = ExporterParameters {
        colors: {
            let mut colors = HashMap::new();
            colors.insert(CubeFace::Front, "#3f0".into());
            colors.insert(CubeFace::Back, "#03c".into());
            colors.insert(CubeFace::Left, "#f90".into());
            colors.insert(CubeFace::Right, "#f30".into());
            colors.insert(CubeFace::Up, "#fff".into());
            colors.insert(CubeFace::Down, "#ff0".into());
            colors
        },
        size: args.resolution as f64,
    };
    exporter.set_params(&params);

    let mut svg_src = Vec::with_capacity(8192);
    exporter.write(&cube, &mut svg_src).unwrap();

    match &args.output_format[..] {
        "svg" => {
            writer.write_all(&svg_src)?;
        }
        "png" => {
            let options = Options::default();
            let tree = Tree::from_data(&svg_src, &options).expect("Valid SVG should be generated");
            let img = resvg::render(&tree, FitTo::Original, None).unwrap();

            let (w, h) = (img.width(), img.height());
            let data = img.take();

            let encoder = PNGEncoder::new(writer);
            encoder
                .encode(&data, w, h, ColorType::Rgba8)
                .expect("PNG data should exist");
        }
        _ => unreachable!(),
    }

    Ok(())
}
