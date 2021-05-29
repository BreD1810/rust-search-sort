use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Array {
    values: Vec<u32>,
    accesses: Vec<usize>,
    sorted_indexes: Vec<usize>,
}

impl Array {
    pub fn new(length: u32) -> Self {
        // Values between 1 and 64 inclusive
        let mut values: Vec<u32> = (1..length + 1).collect();
        values.shuffle(&mut thread_rng());

        let no_values = values.len();
        let accesses = Vec::with_capacity(no_values);
        let sorted_indexes = Vec::with_capacity(no_values);

        Array {
            values,
            accesses,
            sorted_indexes,
        }
    }

    pub fn get(&mut self, index: usize) -> u32 {
        self.accesses.push(index);
        self.values[index]
    }

    pub fn get_without_access(&mut self, index: usize) -> u32 {
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

    pub fn mark_sorted(&mut self, index: usize) {
        self.sorted_indexes.push(index);
    }

    pub fn get_sorted_indexes(&self) -> Vec<usize> {
        self.sorted_indexes.clone()
    }

    pub fn get_accesses(&self) -> Vec<usize> {
        self.accesses.clone()
    }

    pub fn clear_accesses(&mut self) {
        self.accesses.clear()
    }
}
