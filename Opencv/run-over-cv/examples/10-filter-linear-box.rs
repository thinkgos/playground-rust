use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;

    // ! 方框滤波, 线性滤波方式
    // 一种最简单的滤波处理, 它取的是卷积核区域内元素的均值.
    let mut dst = Mat::default();
    imgproc::box_filter(
        &img,                     // 输入图片
        &mut dst,                 // 输出图片
        -1,                       // 深度, -1 表示与输入图片相同
        core::Size::new(3, 3),    // 卷积核大小
        core::Point::new(-1, -1), // 锚点位置, (-1, -1) 表示中心位置
        true,                     // normalize, 是否归一化
        core::BORDER_DEFAULT,     // 边界处理方式
    )?;

    imgcodecs::imwrite("assets/output/lena-filter-box.png", &dst, &Vector::new())?;
    Ok(())
}
