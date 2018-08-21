extern crate rand;
extern crate piston_window;

use std::thread;
use std::time::Duration;
use rand::prelude::*;
use piston_window::*;

const NUM_ROW: u32 = 100;
const NUM_COL: u32 = 100;
const CELL_SIZE: f64 = 5.0;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("piston GoL",[500, 500])
        .build().unwrap();


    // Initialize the state
    let mut state = [0; NUM_ROW as usize * NUM_COL as usize];
    let mut rng = thread_rng();
    for i in 0..state.len() {
       let a: f32 = rng.gen();
       if a > 0.5 {
           state[i] = 1;
       }
    }

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            // Background
            clear([1.0, 1.0, 1.0, 1.0], g);
            // Render the cells
            for i in 0..state.len() {
                let x_pos = i % NUM_COL as usize;
                let y_pos = i / NUM_COL as usize;
                if state[i] == 1 {
                    rectangle([0.0, 0.0, 0.0, 1.0],
                              [x_pos as f64 * CELL_SIZE, y_pos as f64 * CELL_SIZE, CELL_SIZE, CELL_SIZE],
                              c.transform, g);
                }
            }
        });

        // thread::sleep(Duration::from_millis(100));
    }
}
