extern crate wasm_bindgen;
extern crate js_sys;       /* use the js-sys crate to imports the Math.random JavaScript function */
extern crate web_sys;      /* The web-sys crate provides raw wasm-bindgen imports for all of the Web's APIs */

mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;
use web_sys::console;

pub struct Timer<'a> {
        name: &'a str,
}
    
impl<'a> Timer<'a> {
        pub fn new(name: &'a str) -> Timer<'a> {
                console::time_with_label(name);
                Timer { name }
        }
}
    
impl<'a> Drop for Timer<'a> {
        fn drop(&mut self) {
                console::time_end_with_label(self.name);
        }
}
    
fn now() -> f64 {
        web_sys::window()
                .expect("should have a Window")
                .performance()
                .expect("should have a Performance")
                .now()
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/* Dead variant is 0 and Alive variant is 1, so we can easily count a cell's live neighbors with addition */
pub enum Cell {
        Dead = 0,
        Alive = 1,
}

impl Cell {
        fn toggle(&mut self) {
                *self = match *self {
                    Cell::Dead => Cell::Alive,
                    Cell::Alive => Cell::Dead,
                };
        }
}

#[wasm_bindgen]
pub struct Universe {
        width: u32,
        height: u32,
        cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
        /* Constructors */
        pub fn new() -> Universe {
                let width = 128;
                let height = 128;
                Universe::with_capacity(width, height)
        }
        pub fn with_capacity(width: u32, height: u32) -> Universe {
                utils::set_panic_hook();
                let size = (width * height) as usize;
                /* LOGIC DETERMINING INITIAL UNIVERSE CELL STATE */
                let cells = (0..size)
                        .map(|_i| Cell::Dead)
                        .collect();
                Universe {
                        width,
                        height,
                        cells,
                }
        }
        
        /* Getters */
        pub fn width(&self) -> u32 {
                self.width
        }
        pub fn height(&self) -> u32 {
                self.height
        }
        pub fn cells(&self) -> *const Cell {
                self.cells.as_ptr()
        }
        
        /* Setters. Also calls reset() */
        pub fn set_width(&mut self, width: u32) {
                self.width = width;
                self.reset();
        }
        pub fn set_height(&mut self, height: u32) {
                self.height = height;
                self.reset();
        }

        /* Resets all cells in universe to the dead state. */
        pub fn reset(&mut self) {
                /* 
                        This gets an iterator of mutable references to the Cells in our vector.
                        .for_each() > .map() since there are side-effects (mutating an existing cell)
                        For each cell, it dereferences the reference and then changes the Cell to Dead.
                */
                self.cells.iter_mut().for_each(|i| *i = Cell::Dead);
        }

        /* toggle the state of a cell at given row and column */
        pub fn toggle_cell(&mut self, row: u32, column: u32) {
                let idx = self.get_index(row, column);
                self.cells[idx].toggle();
        }

        /* Iterate Universe's State */
        pub fn tick(&mut self) {
                let _timer = Timer::new("Universe::tick");
        
                let mut next = self.cells.clone();
        
                for row in 0..self.height {
                    for col in 0..self.width {
                        let idx = self.get_index(row, col);
                        let cell = self.cells[idx];
                        let live_neighbors = self.live_neighbor_count(row, col);
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
                        /* console.log the row and column of each cell that transitioned states from live to dead or vice versa */
                        if cell != next_cell {
                                log!("cell[{}, {}] is initially {:?} and has {} live neighbors", row,col, cell,live_neighbors);
                        }
        
                        next[idx] = next_cell;
                    }
                }
        
                self.cells = next;
        }

        /* convert Universe State to String */
        pub fn render(&self) -> String {
                self.to_string()
        }

        /* Get linear memory arr_idx from Cell Row & Col number */
        fn get_index(&self, row: u32, column: u32) -> usize {
                (row * self.width + column) as usize
        }
        
        /* Counts # of Neighboring Living Cells  */
        fn live_neighbor_count(&self, row: u32, column: u32) -> u8 { /* I DON'T UNDERSTAND THIS FUNCTION */
                let mut count = 0;

                let north = if row == 0 { self.height - 1 } else { row - 1 };

                let south = if row == self.height - 1 { 0 } else { row + 1 };

                let west  = if column == 0 { self.width - 1 } else { column - 1 };

                let east  = if column == self.width - 1 { 0 } else { column + 1 };

                let nw = self.get_index(north, west);
                count += self.cells[nw] as u8;

                let n = self.get_index(north, column);
                count += self.cells[n] as u8;

                let ne = self.get_index(north, east);
                count += self.cells[ne] as u8;

                let w = self.get_index(row, west);
                count += self.cells[w] as u8;

                let e = self.get_index(row, east);
                count += self.cells[e] as u8;

                let sw = self.get_index(south, west);
                count += self.cells[sw] as u8;

                let s = self.get_index(south, column);
                count += self.cells[s] as u8;

                let se = self.get_index(south, east);
                count += self.cells[se] as u8;

                count
        }
}

/* functions we need for testing that we don't want to expose to our JavaScript */
impl Universe {
        /* Get the dead and alive values of the entire universe.  */
        pub fn get_cells(&self) -> &[Cell] {
                &self.cells
        }
    
        /* Set cells to be alive in a universe by passing the row and column of each cell as an array. */
        pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
                for (row, col) in cells.iter().cloned() {
                        let idx = self.get_index(row, col);
                        self.cells[idx] = Cell::Alive;
                }
        }
    
}

impl fmt::Display for Universe {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                for line in self.cells.as_slice().chunks(self.width as usize) {
                        for &cell in line {
                                let symbol = match cell {
                                        Cell::Dead => '◻',
                                        _ => '◼',
                                };
                                write!(f, "{}", symbol)?;
                        }
                }
                Ok(())
        }
}