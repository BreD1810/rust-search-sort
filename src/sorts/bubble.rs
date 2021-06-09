use super::{Array, Sort};
use std::sync::{Arc, Mutex};

pub struct BubbleSort;

impl Sort for BubbleSort {
    fn sort(&self, array: &Arc<Mutex<Array>>) {
        let len = self.len(array);
        for i in 0..len - 1 {
            let last = len - i - 1;
            for j in 0..last {
                if self.get(array, j) > self.get(array, j + 1) {
                    self.swap(array, j, j + 1);
                    self.wait();
                }
            }
            self.mark_sorted(array, last);
        }
        self.mark_sorted(array, 0);
    }
}
