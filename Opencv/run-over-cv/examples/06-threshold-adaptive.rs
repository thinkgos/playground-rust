use opencv::core::{Mat, Vector};
use opencv::imgcodecs;
use opencv::imgproc;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_GRAYSCALE)?;

    // !自适应域值
    // 参数 1: 要处理的原图
    // 参数 2: 最大阈值,一般为 255
    // 参数 3: 小区域阈值的计算方式
    // ADAPTIVE_THRESH_MEAN_C: 小区域内取均值
    // ADAPTIVE_THRESH_GAUSSIAN_C: 小区域内加权求和,权重是个高斯核
    // 参数 4: 阈值方法,只能使用THRESH_BINARY、THRESH_BINARY_INV,具体见前面所讲的阈值方法
    // 参数 5: 小区域的面积,如 11 就是 11*11 的小块
    // 参数 6: 最终阈值等于小区域计算出的阈值再减去此值
    // 自适应阈值会每次取图片的一小部分计算阈值, 这样图片不同区域的阈值就不尽相同.
    {
        let mut dst = Mat::default();
        imgproc::adaptive_threshold(
            &img,
            &mut dst,
            255.0,
            imgproc::ADAPTIVE_THRESH_MEAN_C,
            imgproc::THRESH_BINARY,
            11,
            4.0,
        )?;
        imgcodecs::imwrite(
            "assets/output/lena-threshold-adaptive-mean.png",
            &dst,
            &Vector::new(),
        )?;
    }

    {
        let mut dst = Mat::default();
        imgproc::adaptive_threshold(
            &img,
            &mut dst,
            255.0,
            imgproc::ADAPTIVE_THRESH_GAUSSIAN_C,
            imgproc::THRESH_BINARY,
            11,
            6.0,
        )?;
        imgcodecs::imwrite(
            "assets/output/lena-threshold-adaptive-gaussian.png",
            &dst,
            &Vector::new(),
        )?;
    }
    Ok(())
}
