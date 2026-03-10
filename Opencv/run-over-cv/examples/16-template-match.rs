use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let mut img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;
    let mut img_gray = Mat::default();
    imgproc::cvt_color(&img, &mut img_gray, imgproc::COLOR_BGR2GRAY, 0)?;

    let tpl = imgcodecs::imread("assets/lena-tpl.png", imgcodecs::IMREAD_GRAYSCALE)?;
    // ! 模板匹配
    // 使用模板匹配在图像中寻找物体, 用于在大图中找小图.
    // TM_SQDIFF: 平方差匹配, 用两者的平方差来匹配, 值越小越相关, 最好的匹配值为0.
    // TM_CCORR: 相关匹配, 用两者的乘积匹配, 数值越大表明匹配程度越好.
    // TM_CCOEFF: 相关系数匹配, 用两者的相关系数匹配, 数值越大表明匹配程度越好, 1 表示完美的匹配, -1 表示最差的匹配.
    // TM_SQDIFF_NORMED: 归一化平方差匹配, 值越小越相关, 最好的匹配值为 0
    // TM_CCORR_NORMED: 归一化相关匹配, 数值越大表明匹配程度越好,  1 表示完美的匹配.
    // TM_CCOEFF_NORMED: 归一化相关系数匹配, 数值越大表明匹配程度越好. 1 表示完美的匹配
    let mut result: Mat = Mat::default();
    imgproc::match_template_def(&img_gray, &tpl, &mut result, imgproc::TM_CCOEFF)?;

    // NOTE: 如果是想要多个模板匹配结果, 需要对结果进行遍历, 按匹配程度过滤出想要的.

    let mut min_val = 0.0;
    let mut max_val = 0.0;
    let mut min_loc = core::Point::new(0, 0);
    let mut max_loc = core::Point::new(0, 0);
    core::min_max_loc(
        &result,
        Some(&mut min_val),
        Some(&mut max_val),
        Some(&mut min_loc),
        Some(&mut max_loc),
        &core::no_array(),
    )?;

    // 在原图上画出矩型
    imgproc::rectangle(
        &mut img,
        core::Rect {
            x: max_loc.x,
            y: max_loc.y,
            width: tpl.cols(),
            height: tpl.rows(),
        },
        core::Scalar::new(0.0, 255.0, 0.0, 0.0),
        2,
        imgproc::LINE_AA,
        0,
    )?;

    imgcodecs::imwrite(
        "assets/output/lena-template-match.png",
        &img,
        &Vector::new(),
    )?;
    Ok(())
}
