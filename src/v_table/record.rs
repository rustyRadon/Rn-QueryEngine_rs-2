pub struct ColumnView<'a> {
    pub data: &'a [i32],
}

pub struct ViewBatch<'a> {
    pub columns: Vec<ColumnView<'a>>,
}