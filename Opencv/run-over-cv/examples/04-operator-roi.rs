use opencv::core::{Rect, Vector};
use opencv::imgcodecs;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;

    // ! ROI
    // 提取ROI区域, (mask也可以使用此方法, 也就是把roi区域设一下)
    let roi = img.roi(Rect {
        x: 120,
        y: 50,
        width: 250,
        height: 350,
    })?;

    imgcodecs::imwrite("assets/output/lena-operator-roi.png", &roi, &Vector::new())?;
    Ok(())
}
