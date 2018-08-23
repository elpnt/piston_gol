extern crate rand;
extern crate rayon;
extern crate piston_window;

mod state;
mod patterns;

use piston_window::*;

const NUM_ROW: usize = 120;
const NUM_COL: usize = 180;
const CELL_SIZE: f64 = 2.0;


fn main() {
    let window_height = NUM_ROW as u32 * CELL_SIZE as u32;
    let window_width = NUM_COL as u32 * CELL_SIZE as u32;
    let mut window: PistonWindow = WindowSettings::new(
        "piston GoL",[window_width, window_height]
        ).build().unwrap();

    let args: Vec<String> = std::env::args().collect();
    let mut state = match args.len() {
        1 => patterns::random_state(NUM_ROW, NUM_COL),
        2 => {
            let pat = &args[1];
            match pat.as_str() {
                "gun" => patterns::glider_gun(NUM_ROW, NUM_COL),
                _     => patterns::random_state(NUM_ROW, NUM_COL),
            }
        },
        _ => {
            println!("The number of arguments must be less than 2.");
            std::process::exit(1);
        }
    };

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
