use itertools::izip;
use js_sys::Uint8Array;
use js_sys::WebAssembly::Memory;
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
    fn evolution(self, live_neighbors: u8) -> Cell {
        match (self, live_neighbors) {
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
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    #[wasm_bindgen(readonly)]
    pub width: u32,
    #[wasm_bindgen(readonly)]
    pub height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Universe {
            width,
            height,
            cells,
        }
    }
    // The result of converting self.cells to Vec<u8> and using Uint8Array::view is incorrect.
    // Handle it directly from wasm memory.
    #[wasm_bindgen(js_name = cells)]
    pub fn cells(&self) -> Uint8Array {
        let buf = wasm_bindgen::memory();
        let mem = buf.unchecked_ref::<Memory>();
        Uint8Array::new_with_byte_offset_and_length(
            &mem.buffer(),
            self.cells.as_ptr() as u32,
            self.cells.len() as u32,
        )
    }
    pub fn render(&self) -> String {
        self.to_string()
    }
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    // Reference from: https://github.com/rustwasm/book/issues/287
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        let width = self.width as usize;
        let height = self.height as usize;

        let next_rows = next.chunks_exact_mut(width);
        let rows_before = self.cells.chunks_exact(width).cycle().skip(height - 1);
        let rows = self.cells.chunks_exact(width);
        let rows_after = self.cells.chunks_exact(width).cycle().skip(1);

        for (next_row, row_b, row, row_a) in izip!(next_rows, rows_before, rows, rows_after) {
            let first_count = row_b[0] as u8
                + row_b[1] as u8
                + row_b[row_b.len() - 1] as u8
                + row[1] as u8
                + row[row.len() - 1] as u8
                + row_a[0] as u8
                + row_a[1] as u8
                + row_a[row_a.len() - 1] as u8;

            next_row[0] = row[0].evolution(first_count);

            let next_cells = next_row.iter_mut().skip(1);
            let window_before = row_b.windows(3);
            let window = row.windows(3);
            let window_after = row_a.windows(3);
            for (next_cell, w_b, w, w_a) in izip!(next_cells, window_before, window, window_after) {
                let count = w_b.iter().map(|&v| v as u8).sum::<u8>()
                    + w[0] as u8
                    + w[2] as u8
                    + w_a.iter().map(|&v| v as u8).sum::<u8>();
                *next_cell = w[1].evolution(count);
            }

            let last_count = row_b[0] as u8
                + row_b[row_b.len() - 2] as u8
                + row_b[row_b.len() - 1] as u8
                + row[0] as u8
                + row[row.len() - 2] as u8
                + row_a[0] as u8
                + row_a[row_a.len() - 2] as u8
                + row_a[row_a.len() - 1] as u8;

            next_row[next_row.len() - 1] = row[row.len() - 1].evolution(last_count);
        }

        self.cells = next;
    }
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }
}

impl Universe {
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }
    pub fn set_cells_alive(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{symbol}")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
