use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;
    // ! 双边滤波, 非线性滤波方式
    // 图像在去噪的同时最高程度保留图形真实度, 边缘信息能有效的保留下来.
    let mut dst = Mat::default();
    imgproc::bilateral_filter(
        &img,                 // 输入图片
        &mut dst,             // 输出图片
        9,                    // 滤波过程中每个像素邻域的直径
        75.0, // 在色彩空间中过滤 sigma.参数值越大,意味着像素邻域内(参见sigmaSpace)更远的颜色将被混合在一起,从而形成更大面积的近似等色区域.
        75.0, // 在坐标空间中过滤 sigma.参数值越大，意味着距离越远的像素只要颜色足够接近就会相互影响(参见sigmaColor).当 d>0 时，它指定邻域大小，与 sigmaSpace 无关.否则，d 与 sigmaSpace 成正比.
        core::BORDER_DEFAULT, // 孔径线性大小, 必须为大于1的奇数
    )?;

    imgcodecs::imwrite(
        "assets/output/lena-filter-bilateral.png",
        &dst,
        &Vector::new(),
    )?;
    Ok(())
}
