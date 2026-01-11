use crate::util::bits::BitMask;
use rayon::prelude::*; 

pub fn run_filter_i32<Rule>(data: &[i32], check_rule: Rule) -> BitMask 
where 
    Rule: Fn(i32) -> bool + Sync + Send 
{
    let total_rows = data.len();
    let mut final_mask = BitMask::new(total_rows);

    data.par_chunks(64)
        .enumerate()
        .map(|(slot_number, row_group)| {
            let mut group_results = 0u64;
            
            for (i, &value) in row_group.iter().enumerate() {
                if check_rule(value) {
                    group_results |= 1 << i;
                }
            }
            (slot_number, group_results)
        })
        .collect::<Vec<(usize, u64)>>()
        .into_iter()
        .for_each(|(slot_number, results)| {

            final_mask.set_holdable(slot_number, results); 
        });

    final_mask
}