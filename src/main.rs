mod conway;
mod lib;


fn main() {
    let conway_board = lib::Board::new(100, Box::new(conway::ConwayRule {}));
}
