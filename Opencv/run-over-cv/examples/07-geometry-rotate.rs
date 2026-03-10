use opencv::core::{self, Mat, MatTraitConst, Vector};
use opencv::{imgcodecs, imgproc};

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;

    // ! 旋转
    // 同平移一样, 也是用仿射变换实现的. 因此需要定义一个变换矩阵.
    let rows = img.rows();
    let cols = img.cols();
    let size = img.size()?;

    // 变换矩阵
    let m = imgproc::get_rotation_matrix_2d(
        core::Point2f::new(cols as f32 / 2.0, rows as f32 / 2.0), // 图片的旋转中心
        45.0, // 旋转角度, (正: 逆时针, 负: 顺时针)
        0.5,  // 缩放比例, 0.5 表示缩小一半
    )?;

    let mut dst = Mat::default();
    imgproc::warp_affine(
        &img,                    // 输入图片
        &mut dst,                // 输出图片
        &m,                      // 变换矩阵 - 旋转矩阵
        size,                    // 输出图片的大小
        imgproc::INTER_LINEAR,   // 插值方法
        core::BORDER_CONSTANT,   // 边界模式
        core::Scalar::default(), // 边界值, 用于CONSTANT边界模式, 默认为0.
    )?;
    imgcodecs::imwrite(
        "assets/output/lena-geometry-rotate.png",
        &dst,
        &Vector::new(),
    )?;
    Ok(())
}
