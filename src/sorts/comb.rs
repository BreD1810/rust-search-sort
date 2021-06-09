use super::{Array, Sort};
use std::sync::{Arc, Mutex};

pub struct CombSort;

impl Sort for CombSort {
    fn sort(&self, array: &Arc<Mutex<Array>>) {
        let len = self.len(array);
        let mut gap = len;
        let shrink = 1.3;
        let mut sorted = false;

        while !sorted {
            gap = (gap as f32 / shrink).floor() as usize;
            if gap <= 1 {
                gap = 1;
                sorted = true;
            }

            let mut i = 0;
            while i + gap < len {
                if self.get(array, i) > self.get(array, i + gap) {
                    self.swap(array, i, i + gap);
                    sorted = false;
                }
                i += 1;
                self.wait();
            }
        }

        for i in 0..len {
            self.mark_sorted(array, i);
        }
    }
}
