use super::{Array, Sort};

pub struct BubbleSort;

impl Sort for BubbleSort {
    fn sort(&self, array: &mut Array) {
        let len = array.len();
        for i in 0..len - 1 {
            let last = len - i - 1;
            for j in 0..last {
                if array.get(j) > array.get(j + 1) {
                    array.swap(j, j + 1);
                }
            }
        }
    }
}
