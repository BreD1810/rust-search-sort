use super::{Array, Sort};
use std::sync::{Arc, Mutex};

pub struct QuickSort;

impl Sort for QuickSort {
    fn sort(&self, array: &Arc<Mutex<Array>>) {
        let high = self.len(array) as isize - 1;
        self.sort_partition(array, 0, high);
    }
}

impl QuickSort {
    fn sort_partition(&self, array: &Arc<Mutex<Array>>, low: isize, high: isize) {
        if low < high {
            let pivot = self.partition(array, low, high);
            self.sort_partition(array, low, pivot - 1);
            self.sort_partition(array, pivot + 1, high);
        } else {
            self.mark_sorted(array, low as usize);
        }
    }

    fn partition(&self, array: &Arc<Mutex<Array>>, low: isize, high: isize) -> isize {
        let pivot = self.get(array, high as usize);
        let mut i = low;
        for j in low..high {
            if self.get(array, j as usize) < pivot {
                self.swap(array, i as usize, j as usize);
                i += 1;
            }
            self.wait();
        }
        self.swap(array, i as usize, high as usize);
        self.mark_sorted(array, i as usize);
        i
    }
}
