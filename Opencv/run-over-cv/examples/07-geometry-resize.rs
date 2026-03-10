use opencv::core::{self, Mat, Vector};
use opencv::imgcodecs;
use opencv::imgproc;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;

    // ! 缩放
    {
        // 按照指定的宽度、高度缩放图片
        let mut dst = Mat::default();
        imgproc::resize(
            &img,                      // 输入图片
            &mut dst,                  // 输出的图片
            core::Size::new(132, 150), // 输出图片的大小
            0.0,                       // 横向轴的比例因子
            0.0,                       // 纵向轴的比例因子
            imgproc::INTER_LINEAR,     // 插值方法
        )?;
        imgcodecs::imwrite(
            "assets/output/lena-geometry-resize-shrink1.png",
            &dst,
            &Vector::new(),
        )?;
    }

    {
        // 按照比例缩放, x,y 轴均缩小一倍, interpolation 采用缩放方法, 默认采用插值
        let mut dst = Mat::default();
        imgproc::resize(
            &img,
            &mut dst,
            core::Size::default(),
            0.5,
            0.5,
            imgproc::INTER_LINEAR,
        )?;
        imgcodecs::imwrite(
            "assets/output/lena-geometry-resize-shrink2.png",
            &dst,
            &Vector::new(),
        )?;
    }

    {
        // 按照比例放大, x,y 轴均放大一倍
        let mut dst = Mat::default();
        imgproc::resize(
            &img,
            &mut dst,
            core::Size::default(),
            2.0,
            2.0,
            imgproc::INTER_LINEAR,
        )?;
        imgcodecs::imwrite(
            "assets/output/lena-geometry-resize-zoom.png",
            &dst,
            &Vector::new(),
        )?;
    }
    Ok(())
}
