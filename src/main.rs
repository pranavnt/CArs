mod conway;
mod gui;
mod eca;

extern crate rand;

use cars::{Board, Cell};

fn main() {
    // conway_test();
    rule30_test()
}

fn rule30_test() {
    let mut rule30_board = Board::new(Box::new(eca::ElementaryRule::new(30)));
    rule30_board.set(50, 0, Cell::Alive);
    rule30_board.snapshot();

    for _ in 0..200 {
        rule30_board.tick();
    }

    rule30_board.export("./videos/eca/30");
}

fn conway_test() {
    let mut conway_board = Board::new(Box::new(conway::ConwayRule {}));

    for i in 40..60 {
        for j in 40..60 {
            // 50% chance of being alive
            if rand::random() {
                conway_board.set(i, j, Cell::Alive);
            }
        }
    }

    conway_board.snapshot();

    for _ in 0..500 {
        conway_board.tick();
        println!("{:?}", conway_board.grid.len())
    }

    conway_board.export("./videos/rule30");
}
