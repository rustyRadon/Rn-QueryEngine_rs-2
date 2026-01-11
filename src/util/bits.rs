pub struct BitMask {
    bits: Vec<u64>,
}

impl BitMask {
    pub fn new(size: usize) -> Self {
        let num_holdable = (size + 63) / 64;
        Self {
            bits: vec![0u64; num_holdable],
        }
    }

    pub fn set(&mut self, index: usize) {
        let holdable_index = index / 64;
        let position = index % 64;
        self.bits[holdable_index] |= 1 << position;
    }

    pub fn get(&self, index: usize) -> bool {
        let holdable_index = index / 64;
        let position = index % 64;
        (self.bits[holdable_index] & (1 << position)) != 0
    }
}