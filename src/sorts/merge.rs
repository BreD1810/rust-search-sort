use super::{Array, Sort};

pub struct MergeSort;

impl Sort for MergeSort {
    fn sort(&self, array: &Array) {
        let len = array.len() - 1;
        self.merge_sort(array, 0, len, true);
    }
}

impl MergeSort {
    fn merge_sort(&self, array: &Array, left: usize, right: usize, last_pass: bool) {
        if left < right {
            let middle = (left + right) / 2;
            self.merge_sort(array, left, middle, false);
            self.merge_sort(array, middle + 1, right, false);
            self.merge(array, left, middle, right, last_pass);
        }
    }

    fn merge(&self, array: &Array, left: usize, middle: usize, right: usize, last_pass: bool) {
        let left_len = middle - left + 1;
        let left_array = get_sub_array(array, left, left_len);
        let right_len = right - middle;
        let right_array = get_sub_array(array, middle + 1, right_len);

        let mut i = 0; // Left index
        let mut j = 0; // Right index
        let mut pos = left;

        while i < left_len && j < right_len {
            if left_array[i] < right_array[j] {
                array.set(pos, left_array[i]);
                i += 1;
            } else {
                array.set(pos, right_array[j]);
                j += 1;
            }
            if last_pass {
                array.mark_sorted(pos);
            }
            pos += 1;
            self.wait();
        }

        while i < left_len {
            array.set(pos, left_array[i]);
            i += 1;
            if last_pass {
                array.mark_sorted(pos);
            }
            pos += 1;
            self.wait();
        }

        while j < right_len {
            array.set(pos, right_array[j]);
            j += 1;
            if last_pass {
                array.mark_sorted(pos);
            }
            pos += 1;
            self.wait();
        }
    }
}

fn get_sub_array(array: &Array, start: usize, size: usize) -> Vec<u32> {
    (0..size).map(|i| array.get(start + i)).collect()
}
