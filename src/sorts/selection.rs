use super::{Array, Sort};

pub struct SelectionSort;

impl Sort for SelectionSort {
    fn sort(&self, array: &Array) {
        let len = array.len();

        for i in 0..len {
            let mut j_min = i;
            for j in i + 1..len {
                if array.get(j) < array.get(j_min) {
                    j_min = j;
                    self.wait();
                }
            }
            if j_min != i {
                array.swap(i, j_min);
                self.wait();
            }
            array.mark_sorted(i);
        }
    }
}
