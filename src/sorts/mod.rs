pub use crate::array::Array;

pub mod bubble;

pub trait Sort {
    fn sort(&self, array: &mut Array);
}
