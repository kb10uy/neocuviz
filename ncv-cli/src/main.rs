use neocuviz::{
    cube::Cube,
    exporter::{Exporter, Fru},
    notation::Movements,
};

use std::io::{prelude::*, stdout, BufWriter};

fn main() {
    let mut cube = Cube::new(3);
    // let movements = Movements::new("x2 y2 R U R' U' R' F R2 U' R' U' R U R' F'");
    let movements = Movements::new("x2 y2 R2 U R' U R' U' R U' R2 U' D R' U R D'");
    for movement in movements {
        cube.apply(movement.unwrap()).unwrap();
    }

    let fru = Fru::new(512.0);
    let mut stdout = BufWriter::new(stdout());
    fru.write(&cube, &mut stdout).unwrap();
}
