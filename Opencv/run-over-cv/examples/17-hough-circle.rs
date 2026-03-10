use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/sun-well.png", imgcodecs::IMREAD_GRAYSCALE)?;

    // 霍夫圆变换
    // 圆是用一般使用霍夫梯度法减少计算量
    // https://en.wikipedia.org/wiki/Circle_Hough_Transform

    let mut edges = Mat::default();
    imgproc::canny_def(
        &img,       // 输入图片
        &mut edges, // 输出图片
        30.0,       // 最低阀值
        70.0,       // 最高阀值
    )?;

    let mut circles: Vector<core::Vec3f> = Vector::new();
    imgproc::hough_circles_def(
        &edges,                  // 要检测的二值图, 一般是阈值分割或边缘检测后的图.
        &mut circles,            // 输出的值.
        imgproc::HOUGH_GRADIENT, // 检测方法, 一般使用霍夫梯度法.
        1.0,                     // dp=1：表示霍夫梯度法中累加器图像的分辨率与原图一致
        20.0,                    // 跟霍夫直线变换中的累加数阈值一样
    )?;

    println!("{:?}", circles.len());

    let mut drawing = Mat::zeros(img.rows(), img.cols(), core::CV_8UC3)?.to_mat()?;
    for p in circles.iter() {
        println!("{:?}", p);
        imgproc::circle_def(
            &mut drawing,
            core::Point {
                x: p[0] as i32,
                y: p[1] as i32,
            },
            p[2] as i32,
            core::Scalar::new(0.0, 0.0, 255.0, 0.0), // 红色
        )?;
    }

    imgcodecs::imwrite(
        "assets/output/shape-hough-circle.png",
        &drawing,
        &Vector::new(),
    )?;
    Ok(())
}
