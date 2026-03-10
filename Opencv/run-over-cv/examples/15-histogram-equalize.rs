use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;

    {
        // ! 直方图均衡化
        // 1. 转换色彩空间到 YCrCb (Y是亮度, Cr/Cb是色度)
        let mut ycrcb = Mat::default();
        imgproc::cvt_color(&img, &mut ycrcb, imgproc::COLOR_BGR2YCrCb, 0)?;
        // 2. 分离通道
        let mut channels = Vector::<Mat>::new();
        core::split(&ycrcb, &mut channels)?;
        // 3. 仅对 Y 通道（索引为 0）进行均衡化
        let mut y_eq = Mat::default();
        imgproc::equalize_hist(&channels.get(0)?, &mut y_eq)?;
        // 把均衡化后的 Y 通道放回去
        channels.set(0, y_eq)?;
        // 4. 合并通道
        let mut merged = Mat::default();
        core::merge(&channels, &mut merged)?;
        // 5. 转回 BGR
        let mut dst = Mat::default();
        imgproc::cvt_color(&merged, &mut dst, imgproc::COLOR_YCrCb2BGR, 0)?;

        imgcodecs::imwrite(
            "assets/output/lena-equalized.png",
            &dst,
            &core::Vector::new(),
        )?;
    }
    {
        // ! 自适应均衡化
        // 1. 转换到 YCrCb 色彩空间
        let mut ycrcb = core::Mat::default();
        imgproc::cvt_color(&img, &mut ycrcb, imgproc::COLOR_BGR2YCrCb, 0)?;

        // 2. 分离通道
        let mut channels = core::Vector::<core::Mat>::new();
        core::split(&ycrcb, &mut channels)?;

        // 3. 创建 CLAHE 处理器
        // clip_limit: 限制对比度的阈值，通常设为 2.0 - 4.0。值越大对比度越强。
        // tile_grid_size: 将图像分为多少个网格进行局部处理，(8, 8) 是经典设置。
        let mut clahe = imgproc::create_clahe(2.0, core::Size::new(8, 8))?;

        // 4. 对 Y 通道（亮度）应用 CLAHE
        let mut y_enhanced = core::Mat::default();
        clahe.apply(&channels.get(0)?, &mut y_enhanced)?;

        // 5. 把增强后的 Y 通道放回向量中
        channels.set(0, y_enhanced)?;

        // 6. 合并通道并转回 BGR
        let mut merged = core::Mat::default();
        core::merge(&channels, &mut merged)?;

        let mut dst = core::Mat::default();
        imgproc::cvt_color(&merged, &mut dst, imgproc::COLOR_YCrCb2BGR, 0)?;

        // 8. 保存结果
        imgcodecs::imwrite("assets/output/lena-clahe.png", &dst, &core::Vector::new())?;
    }
    Ok(())
}
