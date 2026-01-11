#[allow(dead_code)]
pub struct ColumnView<'a> {
    pub data: &'a [i32],
}

#[allow(dead_code)]
pub struct ViewBatch<'a> {
    pub columns: Vec<ColumnView<'a>>,
}

impl<'a> ViewBatch<'a> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_column(&mut self, data: &'a [i32]) {
        self.columns.push(ColumnView { data });
    }
}