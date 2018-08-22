extern crate rand;
extern crate rayon;
extern crate piston_window;

use rand::prelude::*;
use rayon::prelude::*;
use piston_window::*;

const NUM_ROW: u32 = 120;
const NUM_COL: u32 = 200;
const CELL_SIZE: f64 = 2.0;

pub struct State {
    n_row: u32,
    n_col: u32,
    cells: Vec<bool>,
}

fn create_state(n_row: u32, n_col: u32) -> State {
    /*
    let mut cells = vec![false; n_row as usize * n_col as usize];
    let mut rng = thread_rng();
    for i in 0..cells.len() {
       let a: f32 = rng.gen();
       if a > 0.5 {
           cells[i] = true;
       }
    }
    */
    let mut cells = vec![];
    let mut rng = thread_rng();
    for _ in 0 .. n_row*n_col {
        let a: f32 = rng.gen();
        if a > 0.5 {
            cells.push(true);
        } else {
            cells.push(false);
        }
    }

     State {
         n_row,
         n_col,
         cells,
     }
}

impl State {
    fn get_index(&self, row:u32, col: u32) -> usize {
        (row * self.n_col + col) as usize
    }

    fn live_neighbors_count(&self, row: u32, col: u32) -> u8 {
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

    fn next(&mut self) {
        let mut next_cells = self.cells.clone();

        for row in 0 .. self.n_row {
            for col in 0 .. self.n_col {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbors_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (true, x) if x < 2    => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3    => false,
                    (false, 3)            => true,
                    (otherwise, _)        => otherwise,
                };
                next_cells[idx] = next_cell;
            }
        }
        self.cells = next_cells;
    }

}

fn main() {
    let window_height: u32 = NUM_ROW * CELL_SIZE as u32;
    let window_width: u32 = NUM_COL * CELL_SIZE as u32;
    let mut window: PistonWindow = WindowSettings::new(
        "piston GoL",[window_width, window_height]
        ).build().unwrap();

    let mut state = create_state(NUM_ROW, NUM_COL);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            // Background
            clear([1.0, 1.0, 1.0, 1.0], g); // white
            // Render the cells
            for i in 0..state.cells.len() {
                let x_pos = i % NUM_COL as usize;
                let y_pos = i / NUM_COL as usize;
                if state.cells[i] {
                    rectangle([0.0, 0.0, 0.0, 1.0], // black
                              [x_pos as f64 * CELL_SIZE,
                               y_pos as f64 * CELL_SIZE,
                               CELL_SIZE, CELL_SIZE],
                              c.transform, g);
                }
            }
        });
        state.next();
    }
}


#[test]
fn test_get_index() {
    let state = create_state(100, 100);
    assert_eq!(state.get_index(1, 99), 199);
    assert_eq!(state.get_index(99, 98), 9998);
}

#[test]
fn test_live_neighbors_count() {
    let state = State {
        n_row: 5,
        n_col: 5,
        cells: vec![false, true,  false, false, false,
                    true,  true,  false, false, false,
                    false, false, false, false, true,
                    true,  false, false, false, false,
                    false, false, false, true,  true],
    };
    assert_eq!(state.live_neighbors_count(0, 0), 4);
    assert_eq!(state.live_neighbors_count(3, 3), 3);
}

