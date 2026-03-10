use opencv::core::{self, Mat, MatExprTraitConst, Point, Vector};
use opencv::imgcodecs;
use opencv::imgproc;

fn main() -> Result<(), anyhow::Error> {
    // 需要指通颜色通道为 3 通道
    let mut img = Mat::zeros(300, 512, core::CV_8UC3)?.to_mat()?;

    // ! 画多边形
    // 定义四个顶点坐标
    let pts = Vector::from_slice(&[
        Point::new(10, 50),
        Point::new(20, 100),
        Point::new(230, 230),
        Point::new(250, 40),
    ]);
    imgproc::polylines(
        &mut img,                                  // 直接作用的图片
        &pts,                                      // 要画的多边形的顶点坐标
        true,                                      // 是否闭合多边形
        core::Scalar::new(0.0, 255.0, 255.0, 0.0), // 线的颜色
        2,                                         // 线宽
        imgproc::LINE_8,                           // 线的类型
        0,                                         // 偏移量
    )?;
    imgcodecs::imwrite("assets/output/draw-polygon.png", &img, &Vector::new())?;

    // ! 画多条线
    // 可以一次性传多个2*1*2维的数组
    // TODO:
    Ok(())
}
