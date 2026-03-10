use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;
    // ! 高斯滤波, 线性滤波方式
    //  一种基于高斯函数的滤波处理, 它取的是卷积核区域内元素的加权均值.
    //  参数3值越大, 模糊效果越明显.
    let mut dst = Mat::default();
    imgproc::gaussian_blur(
        &img,                  // 输入图片
        &mut dst,              // 输出图片
        core::Size::new(3, 3), // 卷积核大小
        1.0,                   // 卷积核在x方向的标准偏差
        1.0,                   // 卷积核在y方向的标准偏差
        core::BORDER_DEFAULT,  // 边界处理方式
    )?;

    imgcodecs::imwrite(
        "assets/output/lena-filter-gaussian.png",
        &dst,
        &Vector::new(),
    )?;
    Ok(())
}
