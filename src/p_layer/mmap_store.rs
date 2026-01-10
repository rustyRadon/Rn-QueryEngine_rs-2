use bytemuck::checked::cast_slice;
use memmap2::Mmap;
use std::fs::File;
use std::sync::Arc;

pub struct MmapStore {
    pub mmap: Arc<Mmap>,
}

impl MmapStore {
    pub fn new(path: &str) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let mmap = unsafe {Mmap::map(&file).map_err(|e| e.to_string())?};

        Ok(Self {
            mmap: Arc::new(mmap),
        })
    }

    pub fn as_an_i32_slice(&self) -> &[i32] {
        cast_slice(&self.mmap)
    }

    pub fn len(&self) -> usize {
        self.mmap.len()
    }
}