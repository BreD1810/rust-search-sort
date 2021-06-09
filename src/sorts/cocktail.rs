use super::{Array, Sort};

pub struct CocktailSort;

impl Sort for CocktailSort {
    fn sort(&self, array: &Array) {
        let mut start = 0;
        let mut end = array.len() - 1;
        let mut swapped = true;

        while swapped {
            swapped = false;

            for i in start..end {
                if array.get(i) > array.get(i + 1) {
                    array.swap(i, i + 1);
                    swapped = true;
                }
                self.wait();
            }

            if !swapped {
                for i in start..(end + 1) {
                    array.mark_sorted(i);
                }
                break;
            }
            swapped = false;
            array.mark_sorted(end);
            end -= 1;

            for i in (start..end).rev() {
                if array.get(i) > array.get(i + 1) {
                    array.swap(i, i + 1);
                    swapped = true;
                }
                self.wait();
            }

            array.mark_sorted(start);
            start += 1;
        }
    }
}
