use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/sun.png", imgcodecs::IMREAD_GRAYSCALE)?;
    // ! laplacian算子
    // 拉
    let mut dst = Mat::default();
    imgproc::laplacian_def(
        &img,         // 输入图片
        &mut dst,     // 输出图片
        core::CV_64F, // 深度
    )?;
    let dst = core::abs(&dst)?;
    imgcodecs::imwrite(
        "assets/output/sun-operator-laplacian.png",
        &dst,
        &Vector::new(),
    )?;
    Ok(())
}
