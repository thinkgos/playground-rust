use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;
use run_over_cv::imgutil;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;

    // ! 均值滤波, 线性滤波方式
    // 一种最简单的滤波处理, 它取的是卷积核区域内元素的均值.
    let mut dst = Mat::default();
    imgproc::blur(
        &img,                     // 输入图片
        &mut dst,                 // 输出图片
        core::Size::new(3, 3),    // 卷积核大小
        core::Point::new(-1, -1), // 锚点位置, (-1, -1) 表示中心位置
        core::BORDER_DEFAULT,     // 边界处理方式
    )?;

    println!("dst: {:?}", imgutil::shape(&dst));

    imgcodecs::imwrite("assets/output/lena-filter-blur.png", &dst, &Vector::new())?;
    Ok(())
}
