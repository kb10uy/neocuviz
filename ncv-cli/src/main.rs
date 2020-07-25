use neocuviz::{
    cube::Cube,
    exporter::{Exporter, Fru},
    notation::Movements,
};

use std::{
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
    /// キューブのサイズ(分割数)。
    /// デフォルトは 3 (3x3x3 サイズ)
    #[clap(short = "s", long, default_value = "3")]
    cube_size: usize,

    /// 出力ファイルのフォーマットを指定する。
    /// デフォルトは png
    #[clap(short = "f", long, default_value = "svg")]
    output_format: String,

    /// 適用する回転記号列。
    /// 省略された場合は標準入力から読み込む
    movements: Option<String>,

    /// 出力先のファイル名。
    /// 省略された場合は標準出力に書き込む
    output: Option<String>,
}

fn main() -> IoResult<()> {
    let args = Arguments::parse();
    let mut cube = Cube::new(args.cube_size);

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

    let mut movements_str = String::with_capacity(1024);
    reader.read_to_string(&mut movements_str)?;

    // 生成する
    let movements = Movements::new(&movements_str);
    for movement in movements {
        cube.apply(movement.unwrap()).unwrap();
    }

    let fru = Fru::new(512.0);
    let mut svg_src = Vec::with_capacity(8192);
    fru.write(&cube, &mut svg_src).unwrap();

    match &args.output_format[..] {
        "svg" => {
            writer.write_all(&svg_src)?;
        }
        "png" => {
            let options = Options::default();
            let tree = Tree::from_data(&svg_src, &options).expect("Valid SVG should be generated");
            let img_src = resvg::render(&tree, FitTo::Original, None).unwrap();
            img_src.save_png(args.output.unwrap())?;
            /*
            let (w, h) = (img_src.width(), img_src.height());
            let img_data = img_src.take();
            let encoder = PNGEncoder::new(writer);
            encoder
                .encode(&img_data, w, h, ColorType::Rgba8)
                .map_err(|_| IoError::new(ErrorKind::Other, "Unknown format type"))?;
            */
        }
        _ => {
            return Err(IoError::new(ErrorKind::Other, "Unknown format type"));
        }
    }

    Ok(())
}
