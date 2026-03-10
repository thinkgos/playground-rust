use opencv::core::{self, Mat, MatExprTraitConst, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    // 需要指通颜色通道为 3 通道
    let mut img = Mat::zeros(300, 512, core::CV_8UC3)?.to_mat()?;

    let rows = img.rows();
    let cols = img.cols();

    // ! 画椭圆
    // 只需要定义椭圆的中心(x,y), x/y轴的长度, 以及angle旋转角度, startAngle椭圆的起始角度和endAngle椭圆的结束角度, 以及线的颜色和线宽即可.
    // https://stackoverflow.com/questions/48145096/draw-an-arc-by-using-end-points-and-bulge-distance-in-opencv-or-pil
    // angle：
    //  它是椭圆的整体旋转角度.
    //  参考基准是图像的水平 X 轴.
    //  它决定了椭圆的长轴（Major Axis）指向哪个方向.
    // startAngle 和 endAngle (图中蓝色弧线部分)：
    //  它们决定了你要绘制椭圆的哪一段弧线.
    //  关键点： 这两个角度的基准是椭圆的长轴,而不是图像的 X 轴.
    //  如果你设置 startAngle = 0, endAngle = 360,就会画出一个闭合的完整椭圆.
    imgproc::ellipse(
        &mut img,                             // 直接作用的图片
        core::Point::new(cols / 2, rows / 2), // 椭圆的中心
        core::Size::new(200, 100),            // x/y轴的长度
        0.0, // angle旋转角度, 椭圆的整体旋转角度, 参考基准是图像的水平 X 轴, 它决定长轴指向哪个方向
        0.0,
        360.0, // startAngle, endAngle椭圆的起始角度, 结束角度, 决定椭圆弧线的绘制范围, 参考基准是椭圆的长轴(注意不是图像的X轴).
        core::Scalar::new(255.0, 0.0, 0.0, 0.0), // 线的颜色
        -1,    // 线宽, -1表示填充椭圆
        imgproc::LINE_8, // 线的类型
        0,     // 偏移量
    )?;
    imgcodecs::imwrite("assets/output/draw-ellipse.png", &img, &Vector::new())?;
    Ok(())
}
