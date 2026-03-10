use opencv::core::{self, Mat, MatTraitConst, Vector};
use opencv::{imgcodecs, imgproc};

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;

    let size = img.size()?;

    // !平移
    // 用仿射变换实现平移

    // 定义平移矩阵
    // 平移矩阵, x 轴平移 100, y 轴平移 50
    let m: Mat = core::Mat::from_slice_2d(&[[1.0, 0.0, 100.0], [0.0, 1.0, 50.0]])?;

    let mut dst = Mat::default();
    imgproc::warp_affine(
        &img,                    // 输入图片
        &mut dst,                // 输出的图片
        &m,                      // 变换矩阵 - 平移矩阵
        size,                    // 输出图片的大小
        imgproc::INTER_LINEAR,   // 插值方法
        core::BORDER_CONSTANT,   // 边界模式
        core::Scalar::default(), // 边界值, 用于CONSTANT边界模式, 默认为0.
    )?;

    imgcodecs::imwrite(
        "assets/output/lena-geometry-shift.png",
        &dst,
        &Vector::new(),
    )?;
    Ok(())
}
