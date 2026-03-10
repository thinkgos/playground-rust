use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let mut img_color = imgcodecs::imread("assets/sun-well.png", imgcodecs::IMREAD_COLOR)?;

    let mut img_gray = Mat::default();
    imgproc::cvt_color_def(&img_color, &mut img_gray, imgproc::COLOR_BGR2GRAY)?;

    // ! 轮廓特征
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
        &img_thresh,        // 输入图片
        &mut contours,      // 输出图片
        imgproc::RETR_TREE, // 深度
        imgproc::CHAIN_APPROX_SIMPLE,
    )?;

    let cnt = contours.get(1)?;

    // 最小外接矩形
    // 找到一个能包围物体的最小矩形, 注意这是个旋转矩形, 和正矩形是不一样的
    let rotated_rect = imgproc::min_area_rect(&cnt)?;

    let mut rect_points = core::Vec4f::default();
    imgproc::box_points(rotated_rect, &mut rect_points)?;
    println!("min_area_rect points: {:?}", rect_points);

    let mut pts = [core::Point2f::default(); 4];
    rotated_rect.points(&mut pts)?;

    let pts: Vector<core::Point> = pts
        .iter()
        .map(|v| core::Point::new(v.x as i32, v.y as i32))
        .collect();

    imgproc::polylines_def(
        &mut img_color,
        &pts,
        true,
        core::Scalar::new(0.0, 0.0, 255.0, 0.0),
    )?;

    imgcodecs::imwrite(
        "assets/output/sun-contour-rect-rotate.png",
        &img_color,
        &Vector::new(),
    )?;

    Ok(())
}
