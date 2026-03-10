use opencv::core::{self, Mat, MatExprTraitConst, Vector};
use opencv::imgcodecs;
use opencv::imgproc;

fn main() -> Result<(), anyhow::Error> {
    // 需要指通颜色通道为 3 通道
    let mut img = Mat::zeros(300, 512, core::CV_8UC3)?.to_mat()?;
    // !画矩形
    // 只需要定义矩形的左上角和长宽, 以及线的颜色和线宽即可.
    imgproc::rectangle(
        &mut img,                                // 直接作用的图片
        core::Rect::new(50, 100, 350, 100),      // 矩形定义
        core::Scalar::new(0.0, 255.0, 0.0, 0.0), // 线的颜色
        5,                                       // 线宽
        imgproc::LINE_8,                         // 线的类型
        0,                                       // 偏移量
    )?;
    imgcodecs::imwrite("assets/output/draw-rect.png", &img, &Vector::new())?;
    Ok(())
}
