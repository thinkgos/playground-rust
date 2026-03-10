use opencv::core::{Mat, Vector};
use opencv::imgcodecs;
use opencv::imgproc;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;

    // 彩色图转灰度图
    let mut dst = Mat::default();
    imgproc::cvt_color(&img, &mut dst, imgproc::COLOR_BGR2GRAY, 0)?;

    imgcodecs::imwrite("assets/output/lena-convert-gray.png", &dst, &Vector::new())?;
    Ok(())
}
