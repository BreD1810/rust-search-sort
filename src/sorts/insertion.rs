use super::{Array, Sort};

pub struct InsertionSort;

impl Sort for InsertionSort {
    fn sort(&self, array: &Array) {
        let len = array.len();
        for i in 0..len {
            let mut j = i;
            while j > 0 && array.get(j - 1) > array.get(j) {
                array.swap(j, j - 1);
                j -= 1;
                self.wait();
            }
        }

        self.mark_all_sorted(array, len);
    }
}

impl InsertionSort {
    fn mark_all_sorted(&self, array: &Array, len: usize) {
        for i in 0..len {
            array.mark_sorted(i)
        }
    }
}
