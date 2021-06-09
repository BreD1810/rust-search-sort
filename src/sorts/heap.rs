use super::{Array, Sort};
use std::sync::{Arc, Mutex};

pub struct HeapSort;

impl Sort for HeapSort {
    fn sort(&self, array: &Arc<Mutex<Array>>) {
        let n = self.len(array);

        for i in (0..n / 2 - 1).rev() {
            self.heapify(array, n, i);
        }

        for i in (1..n).rev() {
            self.swap(array, 0, i);
            self.mark_sorted(array, i);
            self.wait();
            self.heapify(array, i, 0);
        }

        self.mark_sorted(array, 0);
    }
}

impl HeapSort {
    fn heapify(&self, array: &Arc<Mutex<Array>>, n: usize, i: usize) {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < n && self.get(array, largest) < self.get(array, left) {
            largest = left;
        }

        if right < n && self.get(array, largest) < self.get(array, right) {
            largest = right;
        }

        if largest != i {
            self.swap(array, i, largest);
            self.heapify(array, n, largest);
            self.wait();
        }
    }
}
