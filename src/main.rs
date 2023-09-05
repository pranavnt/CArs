mod conway;
mod lib;


fn main() {
    let mut conway_board = lib::Board::new(100, Box::new(conway::ConwayRule {}));

    // initialize the board
    conway_board.set(50, 50, lib::Cell::Alive);
    conway_board.set(51, 50, lib::Cell::Alive);

    // run the simulation
    for _ in 0..1000 {
        conway_board.tick();
    }

    // render the video
    conway_board.render("./videos/conway.mp4");
}
