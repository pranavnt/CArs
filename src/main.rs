mod conway;
mod gui;
mod rule30;

extern crate rand;

use pulsar::{Board, Cell};

fn main() {
    conway_test();
    // rule30_test()
}

// fn rule30_test() {
//     let mut rule30_board = Board::new(10, Box::new(rule30::Rule30::new()));
//     rule30_board.set(5, 0, Cell::Alive);
//     rule30_board.snapshot();

//     for _ in 0..9 {
//         rule30_board.tick();
//         println!("{:?}", rule30_board.grid);
//     }

//     rule30_board.render("./videos/rule30");
// }

fn conway_test() {
    let mut conway_board = Board::new(Box::new(conway::ConwayRule {}));

    for i in 60..70 {
        for j in 60..70 {
            // 50% chance of being alive
            if rand::random() {
                conway_board.set(i, j, Cell::Alive);
            }
        }
    }

    conway_board.set(10, 10, Cell::Alive);
    conway_board.set(11, 11, Cell::Alive);
    conway_board.set(12, 11, Cell::Alive);
    conway_board.set(12, 10, Cell::Alive);
    conway_board.set(12, 9, Cell::Alive);

    conway_board.snapshot();

    for _ in 0..300 {
        conway_board.tick();
        println!("{:?}", conway_board.grid.len())
    }

    conway_board.render("./videos/conway");
}
