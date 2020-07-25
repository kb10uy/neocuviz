use neocuviz::{
    cube::Cube,
    exporter::{Exporter, Fru},
    notation::Movements,
};

use std::io::{prelude::*, stdout, BufWriter};

fn main() {
    let mut cube = Cube::new(3);
    let movements = Movements::new("R U R' U' R' F R2 U' R' U' R U R' F'");
    for movement in movements {
        cube.apply(movement.unwrap()).unwrap();
    }

    let fru = Fru::new(512.0);
    let mut stdout = BufWriter::new(stdout());
    fru.write(&cube, &mut stdout).unwrap();
}
