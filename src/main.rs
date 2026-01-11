mod kernel; mod util; mod p_layer; mod v_table;

use crate::p_layer::mmap_store::MmapStore;
use crate::p_layer::page_cache::PageCache;
use crate::kernel::filter::run_filter_i32;
use std::time::Instant;
use std::env;
use std::sync::atomic::{AtomicI64, Ordering};
use rayon::prelude::*; 

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let age_limit = args.get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(21);
    let target_dept = args.get(2)
        .map(|s| s.trim_start_matches("--"))
        .unwrap_or("Engineering");
    let sal_limit = args.get(3)
        .and_then(|s| s.parse().ok())  
        .unwrap_or(30000);

    let age_st = MmapStore::new("age.bin")?;
    let sal_st = MmapStore::new("salary.bin")?;
    let dep_st = MmapStore::new("dept.bin")?;
    
    let (ages, sals, deps) = (
        age_st.get(),
        sal_st.get(), 
        dep_st.get()
    );
    let row_count = age_st.row_count();

    let d_names = ["Engineering", "Sales", "Marketing", "HR", "Legal"];
    let target_id = (d_names.iter().position(|&n| n == target_dept).unwrap_or(0) + 1) as i32;

    let timer = Instant::now();

    let mask = run_filter_i32(ages, |age| age > age_limit);

    let total_count = AtomicI64::new(0);
    let total_salary_sum = AtomicI64::new(0);
    let _cache = PageCache::new(1024);

    let morsel_size = 10_000;
    let morsels: Vec<(usize, usize)> = (0..row_count)
        .step_by(morsel_size)
        .map(|start| (start, (start + morsel_size).min(row_count)))
        .collect();

    morsels.into_par_iter().for_each(|(start, end)| {
        let mut local_count = 0;
        let mut local_sum = 0i64;

        for i in start..end {
            if mask.get(i) && deps[i] == target_id && sals[i] > sal_limit {
                local_count += 1;
                local_sum += sals[i] as i64;
            }
        }

        total_count.fetch_add(local_count, Ordering::Relaxed);
        total_salary_sum.fetch_add(local_sum, Ordering::Relaxed);
    });

    let duration = timer.elapsed();
    let final_count = total_count.load(Ordering::Relaxed);
    let final_sum = total_salary_sum.load(Ordering::Relaxed);

    println!("\n--- Parallel Engine Results ---");
    println!("Time: {:?}", duration);
    println!("Matches: {}", final_count);
    if final_count > 0 {
        println!("Avg Salary: ${}", final_sum / final_count);
    }

    Ok(())
}