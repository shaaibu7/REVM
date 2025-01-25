pub mod memory {
    pub struct SimpleMemory {
        memory: Vec<u8>,
    }
    
    impl SimpleMemory {
        pub fn new() -> Self {
            Self { memory: Vec::new() }
        }
    
        pub fn access(&self, offset: usize, size: usize) -> Result<&[u8], String> {
            if offset + size > self.memory.len() {
                return Err(format!("Offset access out of bounds"))
            }
            Ok(&self.memory[offset..offset + size])
        }
    
        pub fn load(&mut self, offset: usize) -> Result<&[u8], String> {
            self.memory.resize(32, 0);
            self.access(offset, 32)
        }
    
        pub fn store(&mut self, offset: usize, value: &[u8]) {
            if offset + value.len() > self.memory.len() {
                self.memory.resize(offset + value.len(), 0);
            }
    
            self.memory[offset..offset + value.len()].copy_from_slice(value);
        }
    
    }
}