use super::{Array, Algorithm};

pub struct GnomeSort;

impl Algorithm for GnomeSort {
    fn run(&self, array: &Array) {
        let mut pos = 0;
        let len = array.len();
        println!("Pos {}", pos);
        println!("Len: {}", len);

        while pos < len {
            if pos == 0 || array.get(pos) >= array.get(pos - 1) {
                pos += 1;
            } else {
                array.swap(pos, pos - 1);
                pos -= 1;
            }

            self.wait();
        }

        for i in 0..len {
            array.mark_final(i);
        }
    }
}
