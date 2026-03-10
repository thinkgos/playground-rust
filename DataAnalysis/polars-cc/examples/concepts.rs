use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = Series::new("ints".into(), &[1, 2, 3, 4, 5]);

    println!("{:?}", s);

    let s1 = Series::new("ints".into(), &[1, 2, 3, 4, 5]);
    let s2 = Series::new("uints".into(), &[1, 2, 3, 4, 5])
        .cast(&DataType::UInt64) // Here, we actually cast after inference.
        .unwrap();
    println!("{} {}", s1.dtype(), s2.dtype()); // i32 u64
    Ok(())
}
