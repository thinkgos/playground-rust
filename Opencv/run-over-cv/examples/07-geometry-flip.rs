use opencv::core::{self, Mat, Vector};
use opencv::imgcodecs;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;

    // ! 翻转
    let mut dst = Mat::default();
    core::flip(
        &img,     // 输入图像
        &mut dst, // 输出图像
        1,        // 翻转方向, = 0 表示垂直翻转, > 0 表示水平翻转, < 0 表示垂直水平翻转
    )?;
    imgcodecs::imwrite(
        "assets/output/lena-geometry-flip-horizontal.png",
        &dst,
        &Vector::new(),
    )?;
    Ok(())
}
