#[cfg(test)]
#[macro_use]
extern crate wasm_bindgen_test;

use bit_vec::BitVec;
use js_sys::Math;
use std::fmt;
use wasm_bindgen::prelude::*;

use utils::set_panic_hook;

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
        let mut count = 0;
        for &dr in [self.height - 1, 0, 1].iter() {
            for &dc in [self.width - 1, 0, 1].iter() {
                if dr == 0 && dc == 0 {
                    continue;
                }

                let idx = self.get_index((row + dr) % self.height, (col + dc) % self.width);
                count += self.cells[idx] as u8;
            }
        }
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

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

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
