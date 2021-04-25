pub struct Array {
    values: Vec<u32>,
}

impl Array {
    pub fn new() -> Self {
        let array = Array {
            values: (1..64).collect(),
        };
        // array.shuffle();

        array
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

    //     use rand::thread_rng;
    //     use rand::seq::SliceRandom;
    //     self.values.shuffle(&mut thread_rng());
    // }
}
