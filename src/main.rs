extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod grid;

use piston_window::PistonWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::collections::HashMap;


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    cells: HashMap<i32, u8>,
}


impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        use grid::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let grid_width: i32 = 20;
        let grid_px_size: i32 = (args.width as i32) / grid_width;
        let cell_width: i32 = 1 * grid_px_size;
        let cell_height: i32 = 1 * grid_px_size;
        let cells = &self.cells;

        println!("{:?} {:?}", args.width, grid_width);


        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            for (pos, lives) in cells {
                let (x, y) = pos_to_xy(*pos, grid_width);
                let x_px_pos = x * cell_width;
                let y_px_pos = y * cell_height;
                println!("pos {:?} x {:?} y {:?} ypx {:?}", pos, x, y, y_px_pos);

                let cell_body = rectangle::square(x_px_pos.into(), y_px_pos.into(), cell_width.into());

                rectangle(RED, cell_body, c.transform, gl);
            }


        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        use grid::*;

        let newCells = &self.cells.clone();
        let grid_width: i32 = 20;

        for (pos, lives) in &self.cells {
            let (x, y) = pos_to_xy(*pos, grid_width);

            newCells.entry(xy_to_pos((x + 1, y), grid_width)).or_insert(1);
            newCells.entry(xy_to_pos((x - 1, y), grid_width)).or_insert(1);
            newCells.entry(xy_to_pos((x, y + 1), grid_width)).or_insert(1);
            newCells.entry(xy_to_pos((x, y - 1), grid_width)).or_insert(1);
        }

        self.cells = *newCells;
    }
}

fn main() {
    let name = "Miner";

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    let width = 200;
    let height = 200;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(name, [width, height])
        .opengl(opengl)
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        cells: HashMap::new()
    };

    let grid_width = 30;

    app.cells.insert(0, 1);
    app.cells.insert(1, 1);
    app.cells.insert(2, 1);
    app.cells.insert(4, 1);
    app.cells.insert(21, 1);
    // app.cells.insert(grid::xy_to_pos((grid_width / 2, (grid_height / 2) + 1), grid_width), 1);
    // app.cells.insert(grid::xy_to_pos((grid_width / 2, (grid_height / 2) + 2), grid_width), 1);

    println!("{:#?}", app.cells);

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
