use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Data as of 14th October 2024, ~3pm UTC
    let df = df!(
        "ticker" => ["AAPL", "NVDA", "MSFT", "GOOG", "AMZN"],
        "company_name" => ["Apple", "NVIDIA", "Microsoft", "Alphabet (Google)", "Amazon"],
        "price" => [229.9, 138.93, 420.56, 166.41, 188.4],
        "day_high" => [231.31, 139.6, 424.04, 167.62, 189.83],
        "day_low" => [228.6, 136.3, 417.52, 164.78, 188.44],
        "year_high" => [237.23, 140.76, 468.35, 193.31, 201.2],
        "year_low" => [164.08, 39.23, 324.39, 121.46, 118.35],
    )?;

    println!("{df}");

    {
        // Explicit expansion by column name
        let eur_usd_rate = 1.09; // As of 14th October 2024

        let df1 = df
            .clone()
            .lazy()
            .with_column(
                (cols(["price", "day_high", "day_low", "year_high", "year_low"]).as_expr()
                    / lit(eur_usd_rate))
                .round(2, RoundMode::default()),
            )
            .collect()?;
        println!("{df1}");
    }

    {
        let eur_usd_rate = 1.09; // As of 14th October 2024
        // Expansion by data type
        let result = df
            .clone()
            .lazy()
            .with_column(
                (dtype_col(&DataType::Float64).as_selector().as_expr() / lit(eur_usd_rate))
                    .round(2, RoundMode::default()),
            )
            .collect()?;
        println!("{result}");
    }
    {
        let result = df
            .clone()
            .lazy()
            .select([all().exclude_cols(["^day_.*$"]).as_expr()])
            .collect()?;
        println!("{result}");
    }
    Ok(())
}
