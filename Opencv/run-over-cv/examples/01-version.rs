use opencv::core;

fn main() -> Result<(), anyhow::Error> {
    // opencv版本
    println!("OpenCV version: {}", core::CV_VERSION);
    Ok(())
}
