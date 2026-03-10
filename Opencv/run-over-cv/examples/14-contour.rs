use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let mut img = imgcodecs::imread("assets/sun-well.png", imgcodecs::IMREAD_COLOR)?;

    let mut img_gray = Mat::default();
    imgproc::cvt_color_def(&img, &mut img_gray, imgproc::COLOR_BGR2GRAY)?;

    // ! 轮廓
    // 轮廓是一系列相连的点组成的曲线, 代表了物体的基本外形.
    // 轮廓是连续的, 边缘并不全都连续.
    // 其实边缘主要是作为图像的特征使用, 比如可以用边缘特征可以区分脸和手,
    // 而轮廓主要用来分析物体的形态, 比如物体的周长和面积等,
    // 可以说边缘包括轮廓

    let mut img_thresh = Mat::default();
    imgproc::threshold(
        &img_gray,                                         // 输入图片
        &mut img_thresh,                                   // 输出图片
        127.0,                                             // 阈值
        255.0,                                             // 最大值
        imgproc::THRESH_BINARY_INV + imgproc::THRESH_OTSU, // 类型
    )?;
    // 寻找二值化图中的轮廓
    let mut contours: Vector<Vector<core::Point>> = Vector::new();
    imgproc::find_contours_def(
        &img_thresh,                  // 输入图片
        &mut contours,                // 输出图片
        imgproc::RETR_TREE, // 轮廓的查找方法, https://docs.opencv.org/4.0.0/d3/dc0/group__imgproc__shape.html#ga819779b9857cc2f8601e6526a3a5bc71
        imgproc::CHAIN_APPROX_SIMPLE, // 轮廓近似方法, https://docs.opencv.org/4.0.0/d3/dc0/group__imgproc__shape.html#ga4303f45752694956374734a03c54d5ff
    )?;
    // 绘制找出来的轮廓
    imgproc::draw_contours(
        &mut img,                                // 输入图片
        &contours,                               // 轮廓
        -1,                                      // 轮廓索引, -1 表示绘制所有轮廓
        core::Scalar::new(0.0, 0.0, 255.0, 0.0), // 颜色
        2,
        imgproc::LINE_8,
        &core::no_array(),
        imgproc::INTER_MAX,
        core::Point::default(),
    )?;

    imgcodecs::imwrite("assets/output/sun-contour.png", &img, &Vector::new())?;

    Ok(())
}
