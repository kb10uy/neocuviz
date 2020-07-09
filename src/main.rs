mod notation;
mod cube;

use crate::notation::Movements;
use std::io::stdin;

fn main() {
    let mut notation = String::new();

    let stdin = stdin();
    while stdin.read_line(&mut notation).unwrap() != 0 {
        let movements = Movements::new(&notation);
        for movement in movements {
            println!("{:?}", movement);
        }
        notation.clear();
    }

    println!("Hello, world!");
}
