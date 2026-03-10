use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ratings = df!(
            "Movie"=> ["Cars", "IT", "ET", "Cars", "Up", "IT", "Cars", "ET", "Up", "Cars"],
            "Theatre"=> ["NE", "ME", "IL", "ND", "NE", "SD", "NE", "IL", "IL", "NE"],
            "Avg_Rating"=> [4.5, 4.4, 4.6, 4.3, 4.8, 4.7, 4.5, 4.9, 4.7, 4.6],
            "Count"=> [30, 27, 26, 29, 31, 28, 28, 26, 33, 28],

    )?;
    println!("{}", &ratings);

    {
        let result = ratings
            .clone()
            .lazy()
            .select([col("Theatre").value_counts(true, true, "count", false)])
            .unnest(by_name(["Theatre"], true), None)
            .collect()?;
        println!("{result}");
    }
    {
        // Don't think we can make it the same way in rust, but this works
        let rating_series = df!(
            "Movie" => &["Cars", "Toy Story"],
            "Theatre" => &["NE", "ME"],
            "Avg_Rating" => &[4.5, 4.9],
        )?
        .into_struct("ratings".into())
        .into_series();
        println!("{}", &rating_series);
        let result = rating_series.struct_()?.field_by_name("Movie")?;
        println!("{result}");
    }
    Ok(())
}
