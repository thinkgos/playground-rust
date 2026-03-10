use chrono::NaiveDate;
use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df: DataFrame = df!(
        "name" => ["Alice Archer", "Ben Brown", "Chloe Cooper", "Daniel Donovan"],
        "birthdate" => [
            NaiveDate::from_ymd_opt(1997, 1, 10).unwrap(),
            NaiveDate::from_ymd_opt(1985, 2, 15).unwrap(),
            NaiveDate::from_ymd_opt(1983, 3, 22).unwrap(),
            NaiveDate::from_ymd_opt(1981, 4, 30).unwrap(),
        ],
        "weight" => [57.9, 72.5, 53.6, 83.1],  // (kg)
        "height" => [1.56, 1.77, 1.65, 1.75],  // (m)
    )?;
    println!("origin --->");
    println!("df {:?}", df);

    {
        let bmi_expr = col("weight") / (col("height").pow(2));
        let df1 = df
            .clone()
            .lazy()
            .with_columns([
                col("birthdate").dt().year().alias("birth_year"),
                bmi_expr.clone().alias("bmi"),
                bmi_expr.clone().mean().alias("bmi_mean"),
                bmi_expr.clone().std(1).alias("bmi_std"),
            ])
            .with_column(((col("bmi") - col("bmi_mean")) / col("bmi_std")).alias("deviation"))
            .collect()?;
        println!("---> first: ");
        println!("{:?}", df1);

        let df2 = df1
            .clone()
            .lazy()
            .group_by([(col("birth_year") / lit(10) * lit(10)).alias("decade")])
            .agg([
                len().alias("sample_size"),
                col("weight")
                    .mean()
                    .round(2, RoundMode::default())
                    .alias("avg_weight"),
                col("weight")
                    .max()
                    .round(2, RoundMode::default())
                    .alias("weightest"),
                col("height")
                    .mean()
                    .round(2, RoundMode::default())
                    .alias("avg_height"),
                col("height").max().alias("tallest"),
            ])
            .collect()?;

        println!("---> second: ");
        println!("{:?}", df2);
    }
    {
        let df1: DataFrame = df!(
            "name" => ["Ben Brown", "Daniel Donovan", "Alice Archer", "Chloe Cooper"],
            "parent" => [true, false, false, false],
            "siblings" => [1, 2, 3, 4],
        )?;
        let df2 = df
            .clone()
            .lazy()
            .join(
                df1.lazy(),
                [col("name")],
                [col("name")],
                JoinArgs::new(JoinType::Left),
            )
            .collect()?;

        println!("---> join: ");
        println!("{:?}", df2);
    }

    Ok(())
}
