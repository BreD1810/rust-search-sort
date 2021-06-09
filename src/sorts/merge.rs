use super::{Array, Sort};
use std::sync::{Arc, Mutex};

pub struct MergeSort;

impl Sort for MergeSort {
    fn sort(&self, array: &Arc<Mutex<Array>>) {
        let len = self.len(array) - 1;
        self.merge_sort(array, 0, len, true);
    }
}

impl MergeSort {
    fn merge_sort(&self, array: &Arc<Mutex<Array>>, left: usize, right: usize, last_pass: bool) {
        if left < right {
            let middle = (left + right) / 2;
            self.merge_sort(array, left, middle, false);
            self.merge_sort(array, middle + 1, right, false);
            self.merge(array, left, middle, right, last_pass);
        }
    }

    fn merge(
        &self,
        array: &Arc<Mutex<Array>>,
        left: usize,
        middle: usize,
        right: usize,
        last_pass: bool,
    ) {
        let left_len = middle - left + 1;
        let left_array = get_sub_array(array, left, left_len);
        let right_len = right - middle;
        let right_array = get_sub_array(array, middle + 1, right_len);

        let mut i = 0; // Left index
        let mut j = 0; // Right index
        let mut pos = left;

        while i < left_len && j < right_len {
            if left_array[i] < right_array[j] {
                self.set(array, pos, left_array[i]);
                i += 1;
            } else {
                self.set(array, pos, right_array[j]);
                j += 1;
            }
            if last_pass {
                self.mark_sorted(array, pos);
            }
            pos += 1;
            self.wait();
        }

        while i < left_len {
            self.set(array, pos, left_array[i]);
            i += 1;
            if last_pass {
                self.mark_sorted(array, pos);
            }
            pos += 1;
            self.wait();
        }

        while j < right_len {
            self.set(array, pos, right_array[j]);
            j += 1;
            if last_pass {
                self.mark_sorted(array, pos);
            }
            pos += 1;
            self.wait();
        }
    }
}

fn get_sub_array(array: &Arc<Mutex<Array>>, start: usize, size: usize) -> Vec<u32> {
    let mut a = array.lock().unwrap();
    (0..size).map(|i| a.get(start + i)).collect()
}
