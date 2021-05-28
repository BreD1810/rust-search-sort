use graphics::color::{TRANSPARENT, WHITE};
use graphics::types::Color;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

use crate::array::Array;
use crate::sorts::bubble::BubbleSort;
use crate::sorts::Sort;

const BACKGROUND_COLOUR: Color = TRANSPARENT;
const VALUE_COLOUR: Color = WHITE;

pub struct App {
    array: Array, // Rotation for the square.
}

impl App {
    pub fn init() -> Self {
        let mut a = Array::new();
        let algo = BubbleSort;
        algo.sort(&mut a);

        App { array: a }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        gl.draw(args.viewport(), |c, gl| {
            use graphics::*;

            clear(BACKGROUND_COLOUR, gl);

            let len = self.array.len();
            let max_val = self.array.max_value() as f64;
            let window_width = args.window_size[0];
            let window_height = args.window_size[1];

            for i in 0..len {
                let value = self.array.get(i);

                let width = window_width / (len as f64);
                let height = f64::from(value) * (window_height / max_val);
                let x = (i as f64) * width;
                let y = window_height - height;

                rectangle(VALUE_COLOUR, [x, y, width, height], c.transform, gl);
            }
        });
    }
}