use super::{Array, Sort};

pub struct CombSort;

impl Sort for CombSort {
    fn sort(&self, array: &Array) {
        let len = array.len();
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
                if array.get(i) > array.get(i + gap) {
                    array.swap(i, i + gap);
                    sorted = false;
                }
                i += 1;
                self.wait();
            }
        }

        for i in 0..len {
            array.mark_sorted(i);
        }
    }
}
