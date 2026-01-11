mod kernel;
mod util;
mod p_layer;
mod v_table;

use crate::p_layer::mmap_store::MmapStore;
use crate::kernel::filter::run_filter_i32;
use std::time::Instant;

fn main() -> Result<(), String> {
    let age_store = MmapStore::new("age.bin")?;
    let salary_store = MmapStore::new("salary.bin")?;
    let dept_store = MmapStore::new("dept.bin")?;

    let ages = age_store.get();
    let salaries = salary_store.get();
    let depts = dept_store.get();

    println!("Starting Industrial Filter (Parallel + Zero-Copy)...");
    let timer = Instant::now();

    let selection_mask = run_filter_i32(ages, |age| age > 21);

    let mut count = 0;
    let mut total_salary: i64 = 0; 

    for i in 0..age_store.row_count() {
        if selection_mask.get(i) {
            // println!(
            //      "Row {}: Age {}, Salary {}, Dept {}", 
            //      i, ages[i], salaries[i], depts[i]
            // );
            total_salary += salaries[i] as i64;
            let _dept_val = depts[i]; 
            
            count += 1;
        }
    }

    let duration = timer.elapsed();
    
    println!("\n--- Performance Results ---");
    println!("Total Rows Processed: {}", age_store.row_count());
    println!("Matches Found:        {}", count);
    println!("Sum of Salaries:      {}", total_salary);
    println!("Execution Time:       {:?}", duration);
    
    Ok(())
}