use super::{Array, Algorithm};

pub struct LinearSearch;

impl Algorithm for LinearSearch {
    fn run(&self, array: &Array) {
        let len = array.len();
        for i in 0..len - 1 {
            if array.get(i) == 80 {
                array.mark_final(i);
                break;
            }
            self.wait();
        }
    }
}
