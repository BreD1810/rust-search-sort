use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Array {
    values: Vec<u32>,
}

impl Array {
    pub fn new() -> Self {
        // Values between 1 and 64 inclusive
        let mut values: Vec<u32> = (1..65).collect();
        values.shuffle(&mut thread_rng());

        Array { values }
    }

    pub fn get(&self, index: usize) -> u32 {
        self.values[index]
    }

    pub fn swap(&mut self, first: usize, second: usize) {
        self.values.swap(first, second);
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn max_value(&self) -> u32 {
        *self.values.iter().max().unwrap_or(&0)
    }
}
