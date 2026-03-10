use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df = df! (
        "col1" => [0.5, 1.0, 1.5, 2.0, 2.5],
        "col2" => [Some(1), None, Some(3), None, Some(5)],
    )?;

    println!("{df}");

    {
        // Fill with a specified literal value
        let fill_literal_df = df
            .clone()
            .lazy()
            .with_column(col("col2").fill_null(3))
            .collect()?;

        println!("{fill_literal_df}");
    }

    {
        // Fill with an expression
        let fill_expression_df = df
            .clone()
            .lazy()
            .with_column(col("col2").fill_null((lit(2) * col("col1")).cast(DataType::Int64)))
            .collect()?;

        println!("{fill_expression_df}");
    }
    {
        // Fill with a strategy based on neighbouring values
        let fill_literal_df = df
            .clone()
            .lazy()
            .with_columns([
                col("col2")
                    .fill_null_with_strategy(FillNullStrategy::Forward(None))
                    .alias("forward"),
                col("col2")
                    .fill_null_with_strategy(FillNullStrategy::Backward(None))
                    .alias("backward"),
            ])
            .collect()?;

        println!("{fill_literal_df}");
    }

    {
        // Fill with interpolation
        let fill_interpolation_df = df
            .lazy()
            .with_column(
                col("col2")
                    .interpolate(InterpolationMethod::Linear)
                    .cast(DataType::Int64),
            )
            .collect()?;

        println!("{fill_interpolation_df}");
    }
    {
        let nan_df = df!(
            "value" => [1.0, f64::NAN, f64::NAN, 3.0],
        )?;
        println!("{nan_df}");
        // not a number
        let mean_nan_df = nan_df
            .lazy()
            .with_column(col("value").fill_nan(Null {}.lit()).alias("replaced"))
            .select([
                col("*").mean().name().suffix("_mean"),
                col("*").sum().name().suffix("_sum"),
            ])
            .collect()?;

        println!("{mean_nan_df}");
    }
    Ok(())
}
