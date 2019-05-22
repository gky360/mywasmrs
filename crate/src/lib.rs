// #![feature(test)]

#[cfg(test)]
#[macro_use]
extern crate wasm_bindgen_test;
// extern crate test;

use bit_vec::BitVec;
use js_sys::Math;
use std::fmt;
use wasm_bindgen::prelude::*;

use utils::{set_panic_hook, Timer};

#[macro_use]
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    cells: BitVec,
}

impl Universe {
    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    fn live_neighbor_count(&self, row: usize, col: usize) -> u8 {
        let north = if row == 0 { self.height - 1 } else { row - 1 };
        let south = if row == self.height - 1 { 0 } else { row + 1 };
        let west = if col == 0 { self.width - 1 } else { col - 1 };
        let east = if col == self.width - 1 { 0 } else { col + 1 };

        let mut count = 0;

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, col);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, col);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new(height: usize, width: usize) -> Universe {
        set_panic_hook();

        let size = width * height;

        let mut cells = BitVec::from_elem(size, false);
        for i in 0..size {
            cells.set(i, Math::random() >= 0.5)
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells_ptr(&self) -> *const u32 {
        self.cells.storage().as_ptr()
    }

    pub fn toggle_cell(&mut self, row: usize, col: usize) {
        let idx = self.get_index(row, col);
        self.cells.set(idx, !self.cells[idx]);
    }

    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");

        let mut next = {
            let _timer = Timer::new("allocate next cells");
            self.cells.clone()
        };

        {
            let _timer = Timer::new("new generation");
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    let live_neighbors = self.live_neighbor_count(row, col);

                    let next_cell = match (cell, live_neighbors) {
                        (true, 2) | (true, 3) => true,
                        (true, _) => false,
                        (false, 3) => true,
                        (cell, _) => cell,
                    };

                    next.set(idx, next_cell);
                }
            }
        }

        let _timer = Timer::new("free old cells");
        self.cells = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                let idx = self.get_index(i, j);
                write!(f, "{}", if self.cells[idx] { "◼" } else { "◻" })?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_cells(universe: &mut Universe, cells: &[(usize, usize)]) {
        universe.cells.clear();
        for &(row, col) in cells.iter() {
            let idx = universe.get_index(row, col);
            universe.cells.set(idx, true);
        }
    }

    fn input_spaceship() -> Universe {
        let mut universe = Universe::new(6, 6);
        set_cells(&mut universe, &[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
        universe
    }

    fn expected_spaceship() -> Universe {
        let mut universe = Universe::new(6, 6);
        set_cells(&mut universe, &[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
        universe
    }

    #[wasm_bindgen_test]
    pub fn test_tick() {
        // Let's create a smaller Universe with a small spaceship to test!
        let mut input_universe = input_spaceship();

        // This is what our spaceship should look like
        // after one tick in our universe.
        let expected_universe = expected_spaceship();

        // Call `tick` and then see if the cells in the `Universe`s are the same.
        input_universe.tick();
        assert_eq!(input_universe.cells, expected_universe.cells);
    }
}

// #[bench]
// fn universe_ticks(b: &mut test::Bencher) {
//     let mut universe = Universe::new(64, 64);

//     b.iter(|| {
//         universe.tick();
//     });
// }
