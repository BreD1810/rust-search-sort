use super::{Array, Sort};
use std::sync::{Arc, Mutex};

pub struct CocktailSort;

impl Sort for CocktailSort {
    fn sort(&self, array: &Arc<Mutex<Array>>) {
        let mut start = 0;
        let mut end = self.len(array) - 1;
        let mut swapped = true;

        while swapped {
            swapped = false;

            for i in start..end {
                if self.get(array, i) > self.get(array, i + 1) {
                    self.swap(array, i, i + 1);
                    swapped = true;
                }
                self.wait();
            }

            if !swapped {
                for i in start..(end + 1) {
                    self.mark_sorted(array, i);
                }
                break;
            }
            swapped = false;
            self.mark_sorted(array, end);
            end -= 1;

            for i in (start..end).rev() {
                if self.get(array, i) > self.get(array, i + 1) {
                    self.swap(array, i, i + 1);
                    swapped = true;
                }
                self.wait();
            }

            self.mark_sorted(array, start);
            start += 1;
        }
    }
}
