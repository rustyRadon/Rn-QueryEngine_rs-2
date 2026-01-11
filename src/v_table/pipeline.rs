#[allow(dead_code)]
pub struct PipelineConfig {
    pub morsel_size: usize,
}

#[allow(dead_code)]
pub struct Pipeline {
    pub config: PipelineConfig,
}

impl Pipeline { 
    #[allow(dead_code)]
    pub fn new(morsel_size: usize) -> Self {
        Self {
            config: PipelineConfig { morsel_size },
        }
    }
    #[allow(dead_code)]
    pub fn execute<F>(&self, total_rows: usize, mut batch_fn: F)
    where
        F: FnMut(usize, usize), 
    {
        let mut start = 0;
        while start < total_rows {
            let end = (start + self.config.morsel_size).min(total_rows);
            batch_fn(start, end);
            start = end;
        }
    }
}

//Morsel-Driven Execution Model. convaying stuff