use super::{Array, Algorithm};

pub struct BinarySearch;

impl Algorithm for BinarySearch {
    fn run(&self, array: &Array) {
        let mut start = 0;
        let mut end = array.len();

        loop {
            let middle = start + ((end - start) / 2);

            let val = array.get(middle);

            match val {
                52 => {
                    array.mark_final(middle);
                    break;
                },
                x if x > 52 => end = middle,
                _ => start = middle,
            }

            for _ in 0..10 {
                self.wait();
            }
        }
    }
}

