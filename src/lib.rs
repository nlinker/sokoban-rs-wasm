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

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    U, D, L, R,
}

#[derive(Debug, Copy, Clone)]
pub enum Cell {
    Worker(Direction),
    WorkerOnGoal(Direction),
    Goal,
    Box,
    BoxOnGoal,
    Empty,
    Wall,
}

impl Cell {
    fn to_u8(self) -> u8 {
        match self {
            Cell::Worker(d) => 10 + (d as u8),
            Cell::WorkerOnGoal(d) => 20 + (d as u8),
            Cell::Goal => 30,
            Cell::Box => 40,
            Cell::BoxOnGoal => 50,
            Cell::Empty => 60,
            Cell::Wall => 70,
        }
    }
}

pub struct GameState {
    height: usize,
    width: usize,
    cells: Vec<Vec<Cell>>,
    is_complete: bool,
}

#[wasm_bindgen]
pub fn get_height() -> usize {
    GS.lock().unwrap().height
}

#[wasm_bindgen]
pub fn get_width() -> usize {
    GS.lock().unwrap().width
}

#[wasm_bindgen]
pub fn get_cells() -> Vec<u8> {
    let gs = &GS.lock().unwrap();
    let mut bs = vec![];
    for i in 0..gs.height {
        for j in 0..gs.width {
            bs.push(gs.cells[i][j].to_u8())
        }
    }
    bs
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_lambdas() {
        let f = || { 5 };
        println!("{:?}", f());
    }
}


