use opencv::core::{self, Mat, MatExprTraitConst, Vector};
use opencv::imgcodecs;
use opencv::imgproc;

fn main() -> Result<(), anyhow::Error> {
    // 需要指通颜色通道为 3 通道
    let mut img = Mat::zeros(300, 512, core::CV_8UC3)?.to_mat()?;

    // !添加文字
    imgproc::put_text(
        &mut img,
        "OpenCv",                                    // 要增加的文本
        core::Point::new(50, 250),                   // # 文字的起始位置(左下角为起始点)
        imgproc::FONT_HERSHEY_SIMPLEX,               // 字体
        2.0,                                         // 文字大小
        core::Scalar::new(255.0, 255.0, 255.0, 0.0), // 文字颜色
        1,                                           // 文字宽度
        imgproc::LINE_AA,                            // 线的类型
        false,                                       // 是否使用反锯齿
    )?;
    imgcodecs::imwrite("assets/output/draw-text.png", &img, &Vector::new())?;
    Ok(())
}
