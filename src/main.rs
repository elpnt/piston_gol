extern crate rand;
extern crate rayon;
extern crate piston_window;

mod state;
mod patterns;

use rayon::prelude::*;
use piston_window::*;

const NUM_ROW: usize = 120;
const NUM_COL: usize = 180;
const CELL_SIZE: f64 = 4.0;


fn main() {
    let window_height = NUM_ROW as u32 * CELL_SIZE as u32;
    let window_width = NUM_COL as u32 * CELL_SIZE as u32;
    let mut window: PistonWindow = WindowSettings::new(
        "piston GoL",[window_width, window_height]
        ).build().unwrap();

    // let mut state = patterns::random_state(NUM_ROW, NUM_COL);
    let mut state = patterns::glider_gun(NUM_ROW, NUM_COL);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            // Background
            clear([0.0, 0.0, 0.0, 1.0], g); // Black 
            // Render the cells
            for i in 0..state.cells.len() {
                let x_pos = i % NUM_COL as usize;
                let y_pos = i / NUM_COL as usize;
                if state.cells[i] {
                    rectangle([1.0, 1.0, 1.0, 1.0], // White 
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

