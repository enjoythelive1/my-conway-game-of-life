mod utils;

use wasm_bindgen::prelude::*;
use js_sys::Math::random;

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
#[derive(Clone)]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<u8>,
}

impl Universe {
    fn get_accessor(&self, row: usize, col: usize) -> (usize, u8) {
        let index = row * self.width() + col;
        let group_index = index / 8;
        let flag = 1 << (index % 8) as u8;

        return (group_index, flag);
    }

    fn live_neighbor_count(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;

        for delta_row in [self.height() - 1, 0, 1].iter() {
            for delta_col in [self.width() - 1, 0, 1].iter() {
                if *delta_row == 0 && *delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + *delta_row) % self.height();
                let neighbor_col = (col + *delta_col) % self.width();
                if let Cell::Alive = self.get_cell(neighbor_row, neighbor_col) {
                    count += 1;
                }
            }
        }

        count
    }

    fn get_cell_memory_size(width: usize, height: usize) -> usize {
        let cells_amount = width * height;
        let size = cells_amount / 8;

        if cells_amount % 8 == 0 {
            size
        } else {
            size + 1
        }
    }

    fn make_random_cells(width: usize, height: usize) -> Vec<u8> {
        (0..Universe::get_cell_memory_size(width, height))
            .map(|_index| (random() * 255f64) as u8)
            .collect()
    }

    pub fn set_cells(&mut self, cells: &[(usize, usize)]) {
        for (row, col) in cells.iter().cloned() {
            self.set_cell(row, col, Cell::Alive);
        }
    }

    pub fn get_cells(&self) -> Vec<Cell> {
        (0..self.height())
            .flat_map(|row| {
                (0..self.width())
                    .map(move |col| self.get_cell(row, col))
            })
            .collect()
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn get_cell(&self, row: usize, col: usize) -> Cell {
        let (index, flag) = self.get_accessor(row, col);

        if self.cells[index] & flag == 0 { Cell::Dead } else { Cell::Alive }
    }

    pub fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
        let (index, flag) = self.get_accessor(row, col);

        match cell {
            Cell::Alive => self.cells[index] |= flag,
            Cell::Dead => self.cells[index] &= !flag,
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.clone();

        for row in 0..self.height() {
            for col in 0..self.width() {
                let cell = self.get_cell(row, col);
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

                next.set_cell(row, col, next_cell);
            }
        }

        self.cells = next.cells;
    }

    pub fn new_random_filled(width: usize, height: usize) -> Universe {
        let cells = Universe::make_random_cells(width, height);

        Universe { cells, width, height }
    }

    pub fn new(width: usize, height: usize) -> Universe {
        let cells = vec![0; Universe::get_cell_memory_size(width, height)];

        Universe { cells, width, height }
    }

    pub fn reset(&mut self) {
        self.cells = Universe::make_random_cells(self.width, self.height)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_gl_cells_buffer(&self) -> Vec<f32> {
        (0..self.height())
            .flat_map(|row| {
                (0..self.width())
                    .map(move |col| (self.get_cell(row, col), row, col))
                    .filter(|(cell, _, _)| *cell == Cell::Alive)
                    .flat_map(|(_, row, col)| {
                        let x = ((col as f32 + 0.5) * 2.0) / self.width() as f32 - 1.0;
                        let y = ((row as f32 + 0.5) * 2.0) / self.height() as f32 - 1.0;

                        vec![x, y]
                    })
            })
            .collect()
    }

    pub fn get_gl_line_buffer(&self) -> Vec<f32> {
        let vertical_lines = (1..self.width()).map(|col| {
            let x = col as f32 * 2.0 / self.width() as f32 - 1.0;

            ((x, -1.0), (x, 1.0))
        });

        let horizontal_lines = (1..self.height()).map(|row| {
            let y = row as f32 * 2.0 / self.height() as f32 - 1.0;

            ((-1.0, y), (1.0, y))
        });

        vertical_lines.chain(horizontal_lines)
            .flat_map(|((px1, py1), (px2, py2))| vec![px1, py1, px2, py2])
            .collect()
    }

    pub fn get_gl_line_vertex_count(&self) -> usize {
        return ((self.width() - 1) + (self.height - 1)) * 2;
    }
}
