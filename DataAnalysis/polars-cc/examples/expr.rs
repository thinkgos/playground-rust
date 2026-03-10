use polars::prelude::*;
use rand::{
    distr::{Distribution, Uniform},
    rng,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df = df! (
        "nrs" => &[Some(1), Some(2), Some(3), None, Some(5)],
        "names" => &["foo", "ham", "spam", "egg", "spam"],
        "random" => &[0.37454, 0.950714, 0.731994, 0.598658, 0.156019],
        "groups" => &["A", "A", "B", "A", "B"],
    )?;

    println!("{}", &df);

    {
        let result = df
            .clone()
            .lazy()
            .select([
                (col("nrs") + lit(5)).alias("nrs + 5"),
                (col("nrs") - lit(5)).alias("nrs - 5"),
                (col("nrs") * col("random")).alias("nrs * random"),
                (col("nrs") / col("random")).alias("nrs / random"),
                (col("nrs").pow(lit(2))).alias("nrs ** 2"),
                (col("nrs") % lit(3)).alias("nrs % 3"),
            ])
            .collect()?;
        println!("first: ");
        println!("{result}");
    }
    {
        let result = df
            .clone()
            .lazy()
            .select([
                col("nrs").gt(1).alias("nrs > 1"),
                col("nrs").gt_eq(3).alias("nrs >= 3"),
                col("random").lt_eq(0.2).alias("random < .2"),
                col("random").lt_eq(0.5).alias("random <= .5"),
                col("nrs").neq(1).alias("nrs != 1"),
                col("nrs").eq(1).alias("nrs == 1"),
            ])
            .collect()?;
        println!("second: ");
        println!("{result}");
    }

    {
        let mut rng = rng();
        let between = Uniform::new_inclusive(0, 100_000).unwrap();
        let arr: Vec<u32> = between.sample_iter(&mut rng).take(100_100).collect();

        let long_df = df!(
            "numbers" => &arr
        )?;

        let result = long_df
            .lazy()
            .select([
                col("numbers").unique_stable().alias("unique"),
                col("numbers").unique_counts().alias("unique_counts"),
                col("numbers").n_unique().alias("n_unique"),
                col("numbers").approx_n_unique().alias("approx_n_unique"),
            ])
            .collect()?;
        println!("unique: ");
        println!("{result}");
    }

    {
        let result = df
            .lazy()
            .select([
                col("nrs"),
                when((col("nrs") % lit(2)).eq(lit(1)))
                    .then(lit(3) * col("nrs") + lit(1))
                    .otherwise(col("nrs") / lit(2))
                    .alias("Collatz"),
            ])
            .collect()?;
        println!("third: ");
        println!("{result}");
    }
    Ok(())
}
