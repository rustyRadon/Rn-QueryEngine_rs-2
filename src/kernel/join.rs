use std::collections::HashMap;

#[allow(dead_code)]
pub fn hash_join(left_col: &[i32], right_col: &[i32]) -> Vec<(usize, usize)> {
    let mut hash_map = HashMap::new();
    let mut results = Vec::new();

    for (i, &val) in left_col.iter().enumerate() {
        hash_map.entry(val).or_insert_with(Vec::new).push(i);
    }

    for (j, &val) in right_col.iter().enumerate() {
        if let Some(left_indices) = hash_map.get(&val) {
            for &i in left_indices {
                results.push((i, j));
            }
        }
    }
    results
}