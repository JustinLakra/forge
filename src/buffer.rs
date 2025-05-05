//Gap buffer
pub struct Buffer {
    vector: Vec<char>,
}

impl Buffer {
    pub fn default() -> Self {
        Buffer { vector: Vec::new() }
    }
    pub fn insert(&mut self, x: u64, y: u64) {}
}
