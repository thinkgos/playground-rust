use opencv::core::{self, Mat, MatExprTraitConst, Vector};
use opencv::imgcodecs;
use opencv::imgproc;

fn main() -> Result<(), anyhow::Error> {
    // 需要指通颜色通道为 3 通道
    let mut img = Mat::zeros(300, 512, core::CV_8UC3)?.to_mat()?;
    // !画圆
    // 只需要定义圆的中心和半径, 以及线的颜色和线宽即可. 这里的线宽为-1表示填充圆.
    imgproc::circle(
        &mut img,                                // 直接作用的图片
        core::Point::new(256, 150),              // 圆的中心
        100,                                     // 圆的半径
        core::Scalar::new(0.0, 0.0, 255.0, 0.0), // 线的颜色
        -1,                                      // 线宽, -1表示填充圆
        imgproc::LINE_8,                         // 线的类型
        0,                                       // 偏移量
    )?;
    imgcodecs::imwrite("assets/output/draw-circle.png", &img, &Vector::new())?;
    Ok(())
}
