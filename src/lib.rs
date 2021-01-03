mod utils;

use std::fmt;

use wasm_bindgen::prelude::*;

use js_sys::Math;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                log!(
                    "cell[{}, {}] is initially {:?} and has {} live neighbors",
                    row,
                    col,
                    cell,
                    live_neighbors
                );

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                log!("    it becomes {:?}", next_cell);

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }
    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;

        //let cells = (0..width * height)
        //    .map(|i| {
        //        if i % 2 == 0 || i % 7 == 0 {
        //            Cell::Alive
        //        } else {
        //            Cell::Dead
        //        }
        //    })
        //    .collect();

        let cells = (0..width * height).map(|i|{
            // let cell = if Math::random()<0.05 { Cell::Alive } else { Cell::Dead };
            let cell = if i % 196 == 55 || i % 196 == (55+64) { Cell::Alive } else { Cell::Dead };
            // let cell = Cell::Dead;
            cell
        }).collect();

        let mut new_universe = Universe {
            width,
            height,
            cells,
        };

        let idx = new_universe.get_index(11,11); new_universe.cells[idx] = Cell::Alive;
        let idx = new_universe.get_index(11,12); new_universe.cells[idx] = Cell::Alive;
        let idx = new_universe.get_index(12,12); new_universe.cells[idx] = Cell::Alive;
        let idx = new_universe.get_index(12,12); new_universe.cells[idx] = Cell::Alive;
        
        let alive_cells = [
            (31,11), (31,12), (32,11), (32,12),
            (21,21),                   (21,24),
                                                (22,25),
            (23,21),                            (23,25),
                     (24,22), (24,23), (24,24), (24,25),
            (22,61), (22,62), (23,61), (23,62),
        ]; 
        let alive_cells2 = [
            (40,30), (40,31), (39,32), (40,33), (40,34),
            (41,30), (41,31),          (41,33), (41,34),
            // (38,32), (37,32), (36,32), (35,32)
            // (37,32), (35,32)
        ];
        for pair in alive_cells.iter() {
            let idx = new_universe.get_index(pair.0, pair.1);
            new_universe.cells[idx] = Cell::Alive;
        }

        new_universe
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

