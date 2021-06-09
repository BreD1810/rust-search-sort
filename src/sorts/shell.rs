use super::{Array, Sort};
use std::sync::{Arc, Mutex};

pub struct ShellSort;

impl Sort for ShellSort {
    fn sort(&self, array: &Arc<Mutex<Array>>) {
        let len = self.len(array);
        let gaps = vec![701, 301, 132, 57, 23, 10, 4, 1];

        for gap in gaps {
            for i in gap..len {
                let temp = self.get(array, i);
                let mut j = i;

                while j >= gap && self.get(array, j - gap) > temp {
                    let val = self.get(array, j - gap);
                    self.set(array, j, val);
                    self.wait();
                    j -= gap;
                }

                self.set(array, j, temp);
                self.wait()
            }
        }

        for i in 0..len {
            self.mark_sorted(array, i);
        }
    }
}
