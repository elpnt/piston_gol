extern crate rand;

use rand::prelude::*;
use state::State;

fn postion_to_index(n_col: usize, row: usize, col: usize) -> usize {
    row * n_col + col
}

pub fn random_state(n_row: usize, n_col: usize) -> State {
    let mut cells = vec![];
    let mut rng = thread_rng();
    for _ in 0..n_row*n_col {
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

pub fn glider_gun(n_row: usize, n_col: usize) -> State {
    let mut cells = vec![false; n_row*n_col];
    let initial_cells = vec![
        // Left Gun
        (5, 1), (5, 2), (6, 1), (6, 2),
        // Right Gun
        (3, 35), (3, 36), (4, 35), (4, 36),
        // Left Bullet
        (5, 11), (6, 11), (7, 11),
        (4, 12), (8, 12),
        (3, 13), (3, 14), (9, 13), (9, 14),
        (6, 15),
        (4, 16), (8, 16),
        (5, 17), (6, 17), (7, 17), (6, 18),
        // Right Bullet
        (3, 21), (4, 21), (5, 21),
        (3, 22), (4, 22), (5, 22),
        (2, 23), (6, 23),
        (1, 25), (2, 25), (6, 25), (7, 25)
    ];

    for position in initial_cells {
        let idx = postion_to_index(n_col, position.0, position.1);
        cells[idx] = true;
    }

    State {
        n_row,
        n_col,
        cells,
    }
}
