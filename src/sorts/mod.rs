pub use crate::array::Array;

pub mod bubble;
pub mod cocktail;
pub mod comb;
pub mod gnome;
pub mod heap;
pub mod insertion;
pub mod merge;
pub mod quick;
pub mod selection;
pub mod shell;

pub trait Sort {
    fn sort(&self, array: &Array);

    fn wait(&self) {
        use std::thread::sleep;
        use std::time::Duration;
        let wait_time = Duration::from_millis(10);
        sleep(wait_time);
    }
}
