mod utils;

use wasm_bindgen::prelude::*;

// start here

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



