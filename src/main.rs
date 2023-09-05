mod conway;

extern crate rand;

use pulsar::{Board, Cell};

fn main() {
    let mut conway_board = Board::new(20, Box::new(conway::ConwayRule {}));

    // initialize the board
    for i in 0..20 {
        for j in 0..20 {
            // 50% chance of being alive
            if rand::random() {
                conway_board.set(i, j, Cell::Alive);
            }
        }
    }

    // take a snapshot of the initial state
    conway_board.snapshot();

    // run the simulation
    for _ in 0..1000 {
        conway_board.tick();
    }

    // render the video
    conway_board.render("./videos");
}
