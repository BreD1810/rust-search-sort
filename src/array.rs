use rand::seq::SliceRandom;
use rand::thread_rng;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
pub struct Array(Arc<Mutex<State>>);

impl Array {
    pub fn new(state: State) -> Self {
        Array(Arc::new(Mutex::new(state)))
    }

    pub fn get_state(&self) -> MutexGuard<'_, State> {
        self.0.lock().unwrap()
    }

    pub fn get(&self, index: usize) -> u32 {
        let mut s = self.get_state();
        s.accesses.push(index);
        s.values[index]
    }

    pub fn get_without_access(&mut self, index: usize) -> u32 {
        let s = self.get_state();
        s.values[index]
    }

    pub fn set(&self, index: usize, value: u32) {
        let mut s = self.get_state();
        s.accesses.push(index);
        s.values[index] = value;
    }

    pub fn len(&self) -> usize {
        let s = self.get_state();
        s.values.len()
    }

    pub fn swap(&self, first: usize, second: usize) {
        let mut s = self.get_state();
        s.values.swap(first, second);
    }

    pub fn mark_final(&self, index: usize) {
        let mut s = self.get_state();
        s.sorted_indexes.push(index);
    }

    pub fn max_value(&self) -> u32 {
        let s = self.get_state();
        *s.values.iter().max().unwrap_or(&0)
    }

    pub fn get_sorted_indexes(&self) -> Vec<usize> {
        let s = self.get_state();
        s.sorted_indexes.clone()
    }

    pub fn get_accesses(&self) -> Vec<usize> {
        let s = self.get_state();
        s.accesses.clone()
    }

    pub fn clear_accesses(&mut self) {
        let mut s = self.get_state();
        s.accesses.clear();
    }
}

pub struct State {
    values: Vec<u32>,
    accesses: Vec<usize>,
    sorted_indexes: Vec<usize>,
}

impl State {
    pub fn new(length: u32, sorted: bool) -> Self {
        // Values between 1 and 64 inclusive
        let mut values: Vec<u32> = (1..length + 1).collect();
        if !sorted {
            values.shuffle(&mut thread_rng());
        }

        let no_values = values.len();
        let accesses = Vec::with_capacity(no_values);
        let sorted_indexes = Vec::with_capacity(no_values);

        State {
            values,
            accesses,
            sorted_indexes,
        }
    }
}
