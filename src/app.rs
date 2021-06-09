use graphics::color::{BLUE, GREEN, TRANSPARENT, WHITE};
use graphics::types::Color;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use std::thread;

use crate::array::{Array, State};
use crate::cli::Parameters;
use crate::algorithms;
use crate::cli::{Search, Sort};

const BACKGROUND_COLOUR: Color = TRANSPARENT;
const VALUE_COLOUR: Color = WHITE;
const SORTED_COLOUR: Color = GREEN;
const ACCESS_COLOUR: Color = BLUE;

pub struct App {
    array: Array,
}

impl App {
    pub fn init(parameters: Parameters) -> Self {
        // Can this be improved?
        // https://doc.rust-lang.org/edition-guide/rust-2018/trait-system/impl-trait-for-returning-complex-types-with-ease.html
        let algo: Box<dyn algorithms::Algorithm + Send> = match parameters.sort {
            Ok(Sort::Bubble) => Box::new(algorithms::bubble::BubbleSort),
            Ok(Sort::Cocktail) => Box::new(algorithms::cocktail::CocktailSort),
            Ok(Sort::Comb) => Box::new(algorithms::comb::CombSort),
            Ok(Sort::Gnome) => Box::new(algorithms::gnome::GnomeSort),
            Ok(Sort::Heap) => Box::new(algorithms::heap::HeapSort),
            Ok(Sort::Insertion) => Box::new(algorithms::insertion::InsertionSort),
            Ok(Sort::Merge) => Box::new(algorithms::merge::MergeSort),
            Ok(Sort::Quick) => Box::new(algorithms::quick::QuickSort),
            Ok(Sort::Selection) => Box::new(algorithms::selection::SelectionSort),
            Ok(Sort::Shell) => Box::new(algorithms::shell::ShellSort),
            Err(_) => match parameters.search {
                Ok(Search::Linear) => Box::new(algorithms::linear::LinearSearch),
                Err(e) => panic!("{}", e),
            }
        };

        let state = match parameters.sort {
            Ok(_) => State::new(parameters.length, false),
            Err(_) => match parameters.search {
                Ok(_) => State::new(parameters.length, true),
                Err(e) => panic!("{}", e),
            }
        };
        let array = Array::new(state);
        let sorting_a = array.clone();


        thread::spawn(move || {
            algo.run(&sorting_a);
        });

        App { array }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        gl.draw(args.viewport(), |c, gl| {
            use graphics::*;

            clear(BACKGROUND_COLOUR, gl);

            let len = self.array.len();
            let max_val = self.array.max_value() as f64;
            let window_width = args.window_size[0];
            let window_height = args.window_size[1];

            let sorted_indexes = self.array.get_sorted_indexes();
            let accesses = self.array.get_accesses();

            for i in 0..len {
                let value = self.array.get_without_access(i);

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
            self.array.clear_accesses();
        });
    }
}
