use bit_vec::BitVec;
use js_sys::Math;
use std::fmt;
use wasm_bindgen::prelude::*;

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
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
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

    pub fn cells(&self) -> *const u32 {
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
