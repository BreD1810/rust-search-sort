use super::{Array, Sort};
use std::sync::{Arc, Mutex};

pub struct GnomeSort;

impl Sort for GnomeSort {
    fn sort(&self, array: &Arc<Mutex<Array>>) {
        let mut pos = 0;
        let len = self.len(array);
        println!("Pos {}", pos);
        println!("Len: {}", len);

        while pos < len {
            if pos == 0 || self.get(array, pos) >= self.get(array, pos - 1) {
                pos += 1;
            } else {
                self.swap(array, pos, pos - 1);
                pos -= 1;
            }

            self.wait();
        }

        for i in 0..len {
            self.mark_sorted(array, i);
        }
    }
}
