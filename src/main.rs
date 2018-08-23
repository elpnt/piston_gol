extern crate rand;
extern crate rayon;
extern crate piston_window;

mod state;
mod patterns;

use piston_window::*;

fn main() {
    let mut n_row: usize = 200;
    let mut n_col: usize = 200;
    let mut cell_size: f64 = 2.0;
    let mut state = patterns::random_state(n_row, n_col);

    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            n_row = 120;
            n_col = 180;
            cell_size = 2.0;
            state = patterns::random_state(n_row, n_col);
        },
        2 => {
            let pat = &args[1];
            match pat.as_str() {
                "gun"   => {
                    n_row = 50;
                    n_col = 200;
                    state = patterns::glider_gun(n_row, n_col);
                },
                "train" => {
                    n_row = 40;
                    n_col = 400;
                    state = patterns::puffer_train(n_row, n_col);
                },
                _       => {}
            }
        },
        _ => {}
    }

    let window_height = n_row as u32 * cell_size as u32;
    let window_width = n_col as u32 * cell_size as u32;
    let mut window: PistonWindow = WindowSettings::new(
        "piston GoL",[window_width, window_height]
        )
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            // Background
            clear([0.0, 0.0, 0.0, 1.0], g); // Black 
            // Render the cells
            for i in 0..state.cells.len() {
                let x_pos = i % n_col as usize;
                let y_pos = i / n_col as usize;
                if state.cells[i] {
                    rectangle([1.0, 1.0, 1.0, 1.0], // White 
                              [x_pos as f64 * cell_size,
                               y_pos as f64 * cell_size,
                               cell_size, cell_size],
                              c.transform, g);
                }
            }
        });
        state.next();
    }
}
