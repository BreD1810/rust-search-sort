use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

mod app;
mod array;
mod cli;
mod sorts;

use crate::app::App;
use crate::cli::parse_parameters;

// Change this to OpenGL::V2_1 if not working
const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

fn main() {
    let parameters = parse_parameters();
    let window_title = format!("{} sort", &parameters.algorithm);

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new(window_title, [640, 480])
        .graphics_api(OPENGL_VERSION)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::init(parameters);
    let mut gl = GlGraphics::new(OPENGL_VERSION);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &mut gl);
        }
    }
}
