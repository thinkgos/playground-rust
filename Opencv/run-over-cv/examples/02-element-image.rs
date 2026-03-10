use opencv::core::{Mat, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use run_over_cv::imgutil;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;
    println!("source shape: {:?}", imgutil::shape(&img));

    // ! 图片颜色转换
    let mut dst = Mat::default();
    imgproc::cvt_color(
        &img,                    // 输入图像
        &mut dst,                // 输出图像
        imgproc::COLOR_BGR2GRAY, // 颜色转换类型
        0,                       // 通道数, 0: 自动检测
    )?;

    imgcodecs::imwrite("assets/output/lena-gray.png", &dst, &Vector::new())?;
    Ok(())
}
