use super::{Array, Sort};
use std::sync::{Arc, Mutex};

pub struct InsertionSort;

impl Sort for InsertionSort {
    fn sort(&self, array: &Arc<Mutex<Array>>) {
        let len = self.len(array);
        for i in 0..len {
            let mut j = i;
            while j > 0 && self.get(array, j - 1) > self.get(array, j) {
                self.swap(array, j, j - 1);
                j -= 1;
                self.wait();
            }
        }

        self.mark_all_sorted(array, len);
    }
}

impl InsertionSort {
    fn mark_all_sorted(&self, array: &Arc<Mutex<Array>>, len: usize) {
        for i in 0..len {
            self.mark_sorted(array, i)
        }
    }
}
