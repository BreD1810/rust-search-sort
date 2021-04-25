use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

mod app;
mod array;

use crate::app::App;

const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

fn main() {
    // Change this to OpenGL::V2_1 if not working

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("static-array", [640, 480])
        .graphics_api(OPENGL_VERSION)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it
    let mut app = App::init();
    let mut gl = GlGraphics::new(OPENGL_VERSION);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &mut gl);
        }
    }
}
