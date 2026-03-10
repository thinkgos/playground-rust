use opencv::core::{self, Mat, MatExprTraitConst, Vector};
use opencv::imgcodecs;
use opencv::imgproc;

fn main() -> Result<(), anyhow::Error> {
    // 需要指定图片的颜色通道为3通道
    let mut img = Mat::zeros(300, 512, core::CV_8UC3)?.to_mat()?;

    // ! 画线
    // 只需要定义起始点和终止点, 以及线的颜色和线宽即可.
    imgproc::line(
        &mut img,                                // 直接作用的图片
        core::Point::new(0, 0),                  // 起始点
        core::Point::new(512, 300),              // 终止点
        core::Scalar::new(255.0, 0.0, 0.0, 0.0), // 线的颜色
        5,                                       // 线宽
        imgproc::LINE_8,                         // 线的类型
        0,                                       // 偏移量
    )?;
    imgcodecs::imwrite("assets/output/draw-line.png", &img, &Vector::new())?;
    Ok(())
}
