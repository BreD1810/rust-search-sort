use super::{Array, Sort};
use std::sync::{Arc, Mutex};

pub struct BubbleSort;

impl Sort for BubbleSort {
    fn sort(&self, array: &Arc<Mutex<Array>>) {
        let len = Self::len(array);
        for i in 0..len - 1 {
            let last = len - i - 1;
            for j in 0..last {
                if Self::get(array, j) > Self::get(array, j + 1) {
                    Self::swap(array, j, j + 1);
                    Self::wait();
                }
            }
            Self::mark_sorted(array, last);
        }
        Self::mark_sorted(array, 0);
    }
}
