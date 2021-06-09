use super::{Array, Sort};
use std::sync::{Arc, Mutex};

pub struct SelectionSort;

impl Sort for SelectionSort {
    fn sort(&self, array: &Arc<Mutex<Array>>) {
        let len = self.len(array);

        for i in 0..len {
            let mut j_min = i;
            for j in i + 1..len {
                if self.get(array, j) < self.get(array, j_min) {
                    j_min = j;
                    self.wait();
                }
            }
            if j_min != i {
                self.swap(array, i, j_min);
                self.wait();
            }
            self.mark_sorted(array, i);
        }
    }
}
