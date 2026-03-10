use opencv::core::Vector;
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;

    // ! 中值滤波, 非线性滤波方式
    // 取所有数排序后取中间的值. 非常适用于去除椒盐噪声和斑点噪声.
    let mut dst = Mat::default();
    imgproc::median_blur(
        &img,     // 输入图片
        &mut dst, // 输出图片
        3,        // 孔径线性大小, 必须为大于1的奇数
    )?;

    imgcodecs::imwrite("assets/output/lena-filter-median.png", &dst, &Vector::new())?;
    Ok(())
}
