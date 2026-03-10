use opencv::core::Vector;
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/shape.jpg", imgcodecs::IMREAD_GRAYSCALE)?;
    // ! 边缘检测
    // 提取步骤:
    // 1. 使用5x5的高斯滤波消除噪声, 平滑处理, 去处噪点.
    // 2. 使用Sobel算子计算图像梯度, 提取水平和垂直方向的边缘.
    // 3. 取局部最大值
    // 4. 滞后阀值
    // 其实可以先阀值分割后再检测边缘, 效果会更好.
    let mut dst = Mat::default();
    imgproc::canny(
        &img,     // 输入图片
        &mut dst, // 输出图片
        30.0,     // 最低阀值
        70.0,     // 最高阀值
        3,        // 孔径大小, 默认3
        false,    //
    )?;

    imgcodecs::imwrite("assets/output/shape-canny.png", &dst, &Vector::new())?;
    Ok(())
}
