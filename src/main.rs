mod lib;

use std::{env::args, path::Path};

use lib::{
    characters::{Guard, Player},
    game::Game,
    grid::Grid,
    parser::GameFile,
};

fn main() {
    let mut args = args();
    if args.len() < 2 {
        panic!("path to .game file not provided");
    }
    let gamefile = GameFile::new(Path::new(&(args.nth(1).unwrap()))).unwrap();
    let mut grid = Grid::new(gamefile.grid_size.0, gamefile.grid_size.1);
    for square in gamefile.custom_squares {
        grid.set_square_wallstate(&square.0, square.1).unwrap();
    }
    let mut game = Game::new(
        Player::new(gamefile.player_position),
        Guard::new(gamefile.guard_position),
        grid,
    );
    game.run();
}
