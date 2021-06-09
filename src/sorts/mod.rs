use std::sync::{Arc, Mutex};

pub use crate::array::Array;

pub mod bubble;
pub mod insertion;
pub mod selection;

pub trait Sort {
    fn sort(&self, array: &Arc<Mutex<Array>>);
    fn get(&self, array: &Arc<Mutex<Array>>, index: usize) -> u32 {
        let mut a = array.lock().unwrap();
        a.get(index)
    }
    fn len(&self, array: &Arc<Mutex<Array>>) -> usize {
        let a = array.lock().unwrap();
        a.len()
    }
    fn swap(&self, array: &Arc<Mutex<Array>>, first: usize, second: usize) {
        let mut a = array.lock().unwrap();
        a.swap(first, second);
    }
    fn mark_sorted(&self, array: &Arc<Mutex<Array>>, index: usize) {
        let mut a = array.lock().unwrap();
        a.mark_sorted(index);
    }
    fn wait(&self) {
        use std::thread::sleep;
        use std::time::Duration;
        let wait_time = Duration::from_millis(10);
        sleep(wait_time);
    }
}
