use super::{Array, Sort};

pub struct ShellSort;

impl Sort for ShellSort {
    fn sort(&self, array: &Array) {
        let len = array.len();
        let gaps = vec![701, 301, 132, 57, 23, 10, 4, 1];

        for gap in gaps {
            for i in gap..len {
                let temp = array.get(i);
                let mut j = i;

                while j >= gap && array.get(j - gap) > temp {
                    let val = array.get(j - gap);
                    array.set(j, val);
                    self.wait();
                    j -= gap;
                }

                array.set(j, temp);
                self.wait()
            }
        }

        for i in 0..len {
            array.mark_sorted(i);
        }
    }
}
