use neocuviz::{
    cube::Cube,
    exporter::{Exporter, Fru},
    notation::Movements,
};

use std::{
    fs::File,
    io::{prelude::*, stdin, stdout, BufReader, BufWriter, Result as IoResult},
};

use clap::Clap;
use usvg::{Tree, FitTo};

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
    let reader: &mut dyn Read = if let Some(filename) = args.movements {
        infile = BufReader::new(File::open(filename)?);
        &mut infile
    } else {
        stdin_instance = BufReader::new(stdin());
        &mut stdin_instance
    };
    let writer: &mut dyn Write = if let Some(filename) = args.output {
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
    let mut svg_src = String::with_capacity(8192);
    fru.write(&cube, &mut svg_src).unwrap();

    match args.output_format {
        "svg" => {
            writer.write_all(svg_src.as_bytes())?;
        }
        "png" => {
            let tree = Tree::from_str(&svg_src).expect("Valid SVG should be generated");
            let img = resvg::render(&tree, usvg::FitTo::Original, None).unwrap();
        }
        _ => {
            return Err("Unknown format");
        }
    }

    Ok(())
}
