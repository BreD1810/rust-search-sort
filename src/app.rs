use graphics::color::{BLUE, GREEN, TRANSPARENT, WHITE};
use graphics::types::Color;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::array::Array;
use crate::cli::Parameters;
use crate::sorts;

const BACKGROUND_COLOUR: Color = TRANSPARENT;
const VALUE_COLOUR: Color = WHITE;
const SORTED_COLOUR: Color = GREEN;
const ACCESS_COLOUR: Color = BLUE;

pub struct App {
    array: Arc<Mutex<Array>>,
}

impl App {
    pub fn init(parameters: Parameters) -> Self {
        let array = Arc::new(Mutex::new(Array::new(parameters.length)));
        let sorting_a = Arc::clone(&array);

        // Can this be improved?
        // https://doc.rust-lang.org/edition-guide/rust-2018/trait-system/impl-trait-for-returning-complex-types-with-ease.html
        let algo: Box<dyn sorts::Sort + Send> = match parameters.algorithm.as_str() {
            "bubble" => Box::new(sorts::bubble::BubbleSort),
            "cocktail" => Box::new(sorts::cocktail::CocktailSort),
            "insertion" => Box::new(sorts::insertion::InsertionSort),
            "merge" => Box::new(sorts::merge::MergeSort),
            "quick" => Box::new(sorts::quick::QuickSort),
            "selection" => Box::new(sorts::selection::SelectionSort),
            "shell" => Box::new(sorts::shell::ShellSort),
            _ => panic!("Unrecognised algorithm: {}", parameters.algorithm),
        };

        thread::spawn(move || {
            algo.sort(&sorting_a);
        });

        App { array }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        gl.draw(args.viewport(), |c, gl| {
            use graphics::*;

            clear(BACKGROUND_COLOUR, gl);

            let mut a = self.array.lock().unwrap();

            let len = a.len();
            let max_val = a.max_value() as f64;
            let window_width = args.window_size[0];
            let window_height = args.window_size[1];

            let sorted_indexes = a.get_sorted_indexes();
            let accesses = a.get_accesses();

            for i in 0..len {
                let value = a.get_without_access(i);

                let width = window_width / (len as f64);
                let height = f64::from(value) * (window_height / max_val);
                let x = (i as f64) * width;
                let y = window_height - height;

                let colour: Color;
                if sorted_indexes.contains(&i) {
                    colour = SORTED_COLOUR;
                } else if accesses.contains(&i) {
                    colour = ACCESS_COLOUR;
                } else {
                    colour = VALUE_COLOUR;
                }

                rectangle(colour, [x, y, width, height], c.transform, gl);
            }
            a.clear_accesses();
        });
    }
}
