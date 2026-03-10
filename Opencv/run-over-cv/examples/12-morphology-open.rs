use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/sun.png", imgcodecs::IMREAD_GRAYSCALE)?;

    // 这个核也叫结构元素, 因为形态学操作其实也是应用卷积来实现的.
    // 结构元素可以是矩形/椭圆/十字形, 可使用get_structuring_element()来生成不同形状的结构元素
    let kernel = Mat::ones(3, 3, core::CV_8U)?.to_mat()?;
    // ! 形态学开运算, 先腐蚀后膨胀
    // 消除物体里的小区域亮点.
    let mut dst = Mat::default();
    imgproc::morphology_ex(
        &img,                                        // 输入图片
        &mut dst,                                    // 输出图片
        imgproc::MORPH_OPEN,                         // 形态学操作类型
        &kernel,                                     // 核
        core::Point::new(-1, -1),                    // 锚点位置, (-1, -1) 表示中心位置
        2,                                           // 操作迭代次数
        core::BORDER_CONSTANT,                       // 边界模式
        imgproc::morphology_default_border_value()?, // 边界值
    )?;

    imgcodecs::imwrite(
        "assets/output/sun-morphology-open.png",
        &dst,
        &Vector::new(),
    )?;
    Ok(())
}
