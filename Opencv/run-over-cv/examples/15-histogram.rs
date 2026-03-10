use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;

    // ! 直方图
    // 直方图就是图像中每个像素值的个数统计
    let mut histograms = vec![];
    for i in 0..3 {
        let mut hist = Mat::default();
        imgproc::calc_hist_def(
            &Vector::<core::Mat>::from_elem(img.clone(), 1), // 输入图片
            &Vector::from_slice(&[i]),                       // 通道
            &Mat::default(),                                 // 掩码
            &mut hist,                                       // 输出直方图值
            &Vector::from_slice(&[256]),
            &Vector::from_slice(&[0.0, 256.0]),
        )?;
        histograms.push(hist);
    }

    println!("hist: {:?}", histograms[0]);
    Ok(())
}
