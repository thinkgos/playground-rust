use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;

    // ! 高斯金字塔

    // 向下采样
    {
        let mut dst = Mat::default();
        imgproc::pyr_down_def(
            &img,     // 输入图片
            &mut dst, // 输出图片
        )?;

        imgcodecs::imwrite("assets/output/lena-pyr-down.png", &dst, &Vector::new())?;
    }

    // 向上采样
    {
        let mut dst = Mat::default();
        imgproc::pyr_up_def(
            &img,     // 输入图片
            &mut dst, // 输出图片
        )?;

        imgcodecs::imwrite("assets/output/lena-pyr-up.png", &dst, &Vector::new())?;
    }

    // ! 拉普拉斯金字塔

    // Li = Gi - pyr_up(py_down(Gi))
    {
        let mut img_down = Mat::default();
        imgproc::pyr_down_def(
            &img,          // 输入图片
            &mut img_down, // 输出图片
        )?;

        let mut img_down_up = Mat::default();
        imgproc::pyr_up_def(
            &img_down,        // 输入图片
            &mut img_down_up, // 输出图片
        )?;

        let dst = core::sub_mat_mat(&img, &img_down_up)?.to_mat()?;
        imgcodecs::imwrite("assets/output/lena-pyr-lpls.png", &dst, &Vector::new())?;
    }

    Ok(())
}
