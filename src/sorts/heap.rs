use super::{Array, Sort};

pub struct HeapSort;

impl Sort for HeapSort {
    fn sort(&self, array: &Array) {
        let n = array.len();

        for i in (0..n / 2 - 1).rev() {
            self.heapify(array, n, i);
        }

        for i in (1..n).rev() {
            array.swap(0, i);
            array.mark_sorted(i);
            self.wait();
            self.heapify(array, i, 0);
        }

        array.mark_sorted(0);
    }
}

impl HeapSort {
    fn heapify(&self, array: &Array, n: usize, i: usize) {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < n && array.get(largest) < array.get(left) {
            largest = left;
        }

        if right < n && array.get(largest) < array.get(right) {
            largest = right;
        }

        if largest != i {
            array.swap(i, largest);
            self.heapify(array, n, largest);
            self.wait();
        }
    }
}
