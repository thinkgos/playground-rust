use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/shape.jpg", imgcodecs::IMREAD_GRAYSCALE)?;

    // 霍夫变换 - 基于统计概率霍夫直线变换

    let mut edges = Mat::default();
    imgproc::canny_def(
        &img,       // 输入图片
        &mut edges, // 输出图片
        30.0,       // 最低阀值
        70.0,       // 最高阀值
    )?;

    let mut lines: Vector<core::Vec4f> = Vector::new();
    imgproc::hough_lines_p(
        &edges,                       // 要检测的二值图, 一般是阈值分割或边缘检测后的图.
        &mut lines,                   // 输出的值.
        1.0,                          // 距离 r 的精度, 值越大, 考虑越多的线.
        std::f64::consts::PI / 180.0, // 角度 θ 的精度, 值越小, 考虑越多的线.
        100,                          // 累加数阈值, 值越小, 考虑越多的线.
        20.0,                         // 最短长度阈值, 比这个长度短的线会被排除.
        10.0,                         // 同一直线两点之间的最大距离
    )?;

    // 绘制直线 (将极坐标转回直角坐标绘制)
    let mut drawing = Mat::zeros(img.rows(), img.cols(), core::CV_8UC3)?.to_mat()?;
    for line in lines.iter() {
        let x1 = line[0];
        let y1 = line[1];
        let x2 = line[2];
        let y2 = line[3];

        let pt1 = core::Point::new(x1 as i32, y1 as i32);
        let pt2 = core::Point::new(x2 as i32, y2 as i32);

        imgproc::line_def(
            &mut drawing,
            pt1,
            pt2,
            core::Scalar::new(0.0, 0.0, 255.0, 0.0), // 红色
        )?;
    }

    imgcodecs::imwrite(
        "assets/output/shape-hough-line-probabilistic.png",
        &drawing,
        &Vector::new(),
    )?;
    Ok(())
}
