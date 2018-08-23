extern crate rayon;

use rayon::prelude::*;

pub struct State {
    pub n_row: usize,
    pub n_col: usize,
    pub cells: Vec<bool>,
}

impl State {

    pub fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.n_col + col
    }

    pub fn index_to_position(&self, idx: usize) -> (usize, usize) {
        let row = idx as usize / self.n_col;
        let col = idx as usize % self.n_col;
        (row, col)
    }

    pub fn live_neighbors_count(&self, row: usize, col: usize) -> u8 {
        let mut count: u8 = 0;
        for i in [self.n_row-1, 0, 1].iter().cloned() {
            for j in [self.n_col - 1, 0, 1].iter().cloned() {
                if i == 0 && j == 0 {
                    continue;
                }

                let neighbor_row = (row + i) % self.n_row;
                let neighbor_col = (col + j) % self.n_col;
                let idx = self.get_index(neighbor_row, neighbor_col);
                if self.cells[idx] {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn next(&mut self) {
        let mut next_cells = self.cells.clone();

        next_cells
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, cell)| {
                let (row, col) = self.index_to_position(i);
                let live_neighbors = self.live_neighbors_count(row, col);
                
                let next_cell = match (*cell, live_neighbors) {
                    (true, x) if x < 2    => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3    => false,
                    (false, 3)            => true,
                    (otherwise, _)        => otherwise,
                };
                *cell = next_cell;
            });

        self.cells = next_cells;
    }
}
