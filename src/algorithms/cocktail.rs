use super::{Array, Algorithm};

pub struct CocktailSort;

impl Algorithm for CocktailSort {
    fn run(&self, array: &Array) {
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
                    array.mark_final(i);
                    println!("Marking {} as sorted", i);
                }
                break;
            }
            swapped = false;
            array.mark_final(end);
            end -= 1;

            for i in (start..end).rev() {
                if array.get(i) > array.get(i + 1) {
                    array.swap(i, i + 1);
                    swapped = true;
                }
                self.wait();
            }

            array.mark_final(start);
            start += 1;

            if !swapped {
                for i in start..(end + 1) {
                    array.mark_final(i);
                    println!("Marking {} as sorted", i);
                }
            }
        }
    }
}
