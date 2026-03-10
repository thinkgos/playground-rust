use opencv::core::{self, Rect, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    // ! 图片混合logo

    // logo原图
    let logo = imgcodecs::imread("assets/logo.png", imgcodecs::IMREAD_COLOR)?;
    let logo_rows = logo.rows();
    let logo_cols = logo.cols();
    // logo灰度图
    let mut logo_gray = Mat::default();
    imgproc::cvt_color(&logo, &mut logo_gray, imgproc::COLOR_BGR2GRAY, 0)?;
    {
        // 方法1
        // logo块阈值处理, 生成掩膜
        // 这里是把logo扣出来, 用于处理真正图片背景色
        let mut mask_inv = Mat::default();
        imgproc::threshold(
            &logo_gray,
            &mut mask_inv,
            127.0,
            255.0,
            imgproc::THRESH_BINARY_INV,
        )?;

        // 处理原图
        let mut img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;

        // 提取原图的roi区域, 用于处理logo
        let mut roi = img.roi_mut(Rect {
            x: 0,
            y: 0,
            width: logo_cols,
            height: logo_rows,
        })?;
        // 保留原图除logo外的背景
        // NOTE: 这样做的原因是logo有可能是透明的, 透明部分不应该被替换.
        let mut roi_bg = Mat::default();
        core::bitwise_and(&roi, &roi, &mut roi_bg, &mask_inv)?;

        // 图片叠加, 相加的两幅图片的形状必须相同.
        let mut roi_mix_logo = Mat::default();
        core::add(
            &roi_bg,           // 输入图1
            &logo,             // 输入图2
            &mut roi_mix_logo, // 输出图
            &core::no_array(), // 可选操作掩码
            -1,                // 可选深度
        )?;
        // 只改变roi区域, 也就是logo位
        roi_mix_logo.copy_to(&mut roi)?;

        imgcodecs::imwrite("assets/output/lena-mixture-logo1.png", &img, &Vector::new())?;
    }

    {
        // 方法2
        // logo块阈值处理, 生成掩膜
        // 这里是把logo扣出来, 用于处理真正图片背景色
        let mut mask = Mat::default();
        imgproc::threshold(&logo_gray, &mut mask, 127.0, 255.0, imgproc::THRESH_BINARY)?;
        // 处理原图
        let mut img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;

        // 提取原图的roi区域, 用于处理logo
        let mut roi = img.roi_mut(Rect {
            x: 0,
            y: 0,
            width: logo_cols,
            height: logo_rows,
        })?;
        logo.copy_to_masked(&mut roi, &mask)?;

        imgcodecs::imwrite("assets/output/lena-mixture-logo2.png", &img, &Vector::new())?;
    }
    Ok(())
}
