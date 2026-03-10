use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/shape.jpg", imgcodecs::IMREAD_GRAYSCALE)?;

    // 霍夫变换
    // 霍夫变换常用来在图像中提取直线和圆等几何形状
    // 直线可以分别用直角坐标系和极坐标系来表示:
    // 经过某个点(x0,y0)的所有直线都可以用这个式子来表示: rθ​=x0 * cosθ + y0 * sinθ
    // 也就是说每一个 (r,θ) 都表示一条经过 (x0,y0) 直线, 那么同一条直线上的点必然会有同样的 (r,θ)。
    // 如果将某个点所有的(r,θ)绘制成曲线, 那么同一条直线上的点的(r,θ)曲线会相交于一点：
    // OpenCV 中首先计算 (r,θ) 累加数, 累加数超过一定值后就认为在同一直线上

    let mut edges = Mat::default();
    imgproc::canny_def(
        &img,       // 输入图片
        &mut edges, // 输出图片
        30.0,       // 最低阀值
        70.0,       // 最高阀值
    )?;

    let mut lines: Vector<core::Vec2f> = Vector::new();
    imgproc::hough_lines_def(
        &edges,                       // 要检测的二值图, 一般是阈值分割或边缘检测后的图.
        &mut lines,                   // 输出的值.
        1.0,                          // 距离 r 的精度, 值越大, 考虑越多的线.
        std::f64::consts::PI / 180.0, // 角度 θ 的精度, 值越小, 考虑越多的线.
        100,                          // 累加数阈值, 值越小, 考虑越多的线.
    )?;

    // 绘制直线(将极坐标 -> 直角坐标绘制)
    let mut drawing = Mat::zeros(img.rows(), img.cols(), core::CV_8UC3)?.to_mat()?;
    for line in lines.iter() {
        let rho = line[0]; // r
        let theta = line[1]; // θ

        let a = theta.cos();
        let b = theta.sin();
        let x0 = a * rho;
        let y0 = b * rho;

        // 计算直线上的两个点（用于绘制长线）
        let pt1 = core::Point::new((x0 + 1000.0 * (-b)) as i32, (y0 + 1000.0 * (a)) as i32);
        let pt2 = core::Point::new((x0 - 1000.0 * (-b)) as i32, (y0 - 1000.0 * (a)) as i32);

        imgproc::line_def(
            &mut drawing,
            pt1,
            pt2,
            core::Scalar::new(0.0, 0.0, 255.0, 0.0), // 红色
        )?;
    }

    imgcodecs::imwrite(
        "assets/output/shape-hough-line-std.png",
        &drawing,
        &Vector::new(),
    )?;
    Ok(())
}
