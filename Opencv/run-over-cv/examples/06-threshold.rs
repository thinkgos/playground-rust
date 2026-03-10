use opencv::core::{Mat, Vector};
use opencv::imgcodecs;
use opencv::imgproc;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_GRAYSCALE)?;

    // ! 固定阀值
    // 图像二值化, 就是像素点值大于阈值变成一类值, 小于阈值变成另一类值
    {
        let mut dst = Mat::default();
        imgproc::threshold(
            &img,                   // 输入图像
            &mut dst,               // 输出图像
            127.0,                  // 阈值
            255.0, // 使用THRESH_BINARY、THRESH_BINARY_INV阈值方法所选用的最大阈值, 一般为255
            imgproc::THRESH_BINARY, // 阈值方式, 有五种
        )?;
        imgcodecs::imwrite(
            "assets/output/lena-threshold-binary.png",
            &dst,
            &Vector::new(),
        )?;
    }

    {
        let mut dst = Mat::default();
        imgproc::threshold(&img, &mut dst, 127.0, 255.0, imgproc::THRESH_BINARY_INV)?;
        imgcodecs::imwrite(
            "assets/output/lena-threshold-binary-inv.png",
            &dst,
            &Vector::new(),
        )?;
    }

    {
        let mut dst = Mat::default();
        imgproc::threshold(&img, &mut dst, 127.0, 255.0, imgproc::THRESH_TRUNC)?;
        imgcodecs::imwrite(
            "assets/output/lena-threshold-trunc.png",
            &dst,
            &Vector::new(),
        )?;
    }
    {
        let mut dst = Mat::default();
        imgproc::threshold(&img, &mut dst, 127.0, 255.0, imgproc::THRESH_TOZERO)?;
        imgcodecs::imwrite(
            "assets/output/lena-threshold-tozero.png",
            &dst,
            &Vector::new(),
        )?;
    }
    {
        let mut dst = Mat::default();
        imgproc::threshold(&img, &mut dst, 127.0, 255.0, imgproc::THRESH_TOZERO_INV)?;
        imgcodecs::imwrite(
            "assets/output/lena-threshold-tozero-inv.png",
            &dst,
            &Vector::new(),
        )?;
    }

    Ok(())
}
