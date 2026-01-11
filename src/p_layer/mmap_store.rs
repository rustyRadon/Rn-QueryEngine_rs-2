use bytemuck::cast_slice;
use memmap2::Mmap;
use std::fs::File;
use std::sync::Arc;

pub struct MmapStore {
    pub raw_bytes: Arc<Mmap>,
}

impl MmapStore {
    pub fn new(path: &str) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let mmap = unsafe { Mmap::map(&file).map_err(|e| e.to_string())? };

        Ok(Self {
            raw_bytes: Arc::new(mmap),
        })
    }

    pub fn get(&self) -> &[i32] {
        cast_slice(&self.raw_bytes)
    }

    pub fn row_count(&self) -> usize {
        self.raw_bytes.len() / 4 
    }
}