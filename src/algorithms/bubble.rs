use super::{Array, Algorithm};

pub struct BubbleSort;

impl Algorithm for BubbleSort {
    fn run(&self, array: &Array) {
        let len = array.len();
        for i in 0..len - 1 {
            let last = len - i - 1;
            for j in 0..last {
                if array.get(j) > array.get(j + 1) {
                    array.swap(j, j + 1);
                    self.wait();
                }
            }
            array.mark_final(last);
        }
        array.mark_final(0);
    }
}
