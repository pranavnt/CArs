mod conway;
mod gui;
mod rule30;

extern crate rand;

use pulsar::{Board, Cell};

fn main() {
    // Create the initial board state. Adjust parameters as necessary.
    let initial_board = board::Board::new(50, Box::new(board::ConwayRule {}));

    // Launch the GUI application.
    gui::launch(initial_board);
    // conway_test();
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
    let mut conway_board = Board::new(50, Box::new(conway::ConwayRule {}));

    for i in 0..50 {
        for j in 0..50 {
            // 50% chance of being alive
            if rand::random() {
                conway_board.set(i, j, Cell::Alive);
            }
        }
    }

    conway_board.snapshot();

    for _ in 0..1000 {
        conway_board.tick();
    }

    // conway_board.render("./videos/conway");
}
