mod utils;

use std::sync::Mutex;
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;

// start here
static GS: Lazy<Mutex<GameState>> = Lazy::new(|| {
    // ####
    // # .#
    // #  ###
    // #*@  #
    // #  $ #
    // #  ###
    // ####
    use Cell::*;
    use Direction::*;
    let cells = vec![
        vec![Wall, Wall, Wall, Wall, Empty, Empty],
        vec![Wall, Empty, Goal, Wall, Empty, Empty],
        vec![Wall, Empty, Empty, Wall, Wall, Empty],
        vec![Wall, BoxOnGoal, Worker(D), Empty, Empty, Wall],
        vec![Wall, Empty, Empty, Box, Empty, Wall],
        vec![Wall, Empty, Empty, Wall, Wall, Wall],
        vec![Wall, Wall, Wall, Wall, Empty, Empty],
    ];
    let gs = GameState {
        height: 7,
        width: 6,
        cells,
        is_complete: false,
    };
    Mutex::new(gs)
});

pub enum Direction {
    U, D, L, R,
}

pub enum Cell {
    Worker(Direction),
    WorkerOnGoal(Direction),
    Goal,
    Box,
    BoxOnGoal,
    Empty,
    Wall,
}

pub struct GameState {
    height: usize,
    width: usize,
    cells: Vec<Vec<Cell>>,
    is_complete: bool,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lambdas() {
        let f = || { 5 };
        println!("{:?}", f());
    }
}


