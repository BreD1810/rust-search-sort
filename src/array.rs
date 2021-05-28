use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct Array {
    values: Vec<u32>,
}

impl Array {
    pub fn new() -> Self {
        let mut values: Vec<u32> = (1..64).collect();
        values.shuffle(&mut thread_rng());

        Array {
            values
        }
    }

    pub fn get(&self, index:usize) -> u32 {
        self.values[index]
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn max_value(&self) -> u32 {
        *self.values.iter().max().unwrap_or(&0)
    }

}
