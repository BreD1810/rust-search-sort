use super::{Array, Sort};

pub struct QuickSort;

impl Sort for QuickSort {
    fn sort(&self, array: &Array) {
        let high = array.len() as isize - 1;
        self.sort_partition(array, 0, high);
    }
}

impl QuickSort {
    fn sort_partition(&self, array: &Array, low: isize, high: isize) {
        if low < high {
            let pivot = self.partition(array, low, high);
            self.sort_partition(array, low, pivot - 1);
            self.sort_partition(array, pivot + 1, high);
        } else {
            array.mark_sorted(low as usize);
        }
    }

    fn partition(&self, array: &Array, low: isize, high: isize) -> isize {
        let pivot = array.get(high as usize);
        let mut i = low;
        for j in low..high {
            if array.get(j as usize) < pivot {
                array.swap(i as usize, j as usize);
                i += 1;
            }
            self.wait();
        }
        array.swap(i as usize, high as usize);
        array.mark_sorted(i as usize);
        i
    }
}
