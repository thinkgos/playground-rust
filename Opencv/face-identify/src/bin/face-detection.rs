use face_identify::visualize::DetectionVisualizeColor;
use face_identify::yu_net::Detection;
use opencv::core::Vector;
use opencv::prelude::*;

use face_identify::yu_net::YuNet;

fn main() -> Result<(), anyhow::Error> {
    let mut detector = YuNet::new("../model/face_detection_yunet_2023mar.onnx")?;

    let mut image = opencv::imgcodecs::imread("assets/face1.jpg", opencv::imgcodecs::IMREAD_COLOR)?;

    detector.set_input_size(image.size()?)?;
    let (ret, faces) = detector.detect(&image)?;
    if ret == 0 {
        println!("No face detected.");
        return Ok(());
    } else {
        println!("Detected faces: {:?}", ret);
    }

    // 获取一行检测结果
    let row = faces.at_row::<f32>(0)?;
    let detection: Detection = row.try_into()?;

    let dvc = DetectionVisualizeColor::default();
    dvc.draw(&mut image, &detection)?;

    opencv::imgcodecs::imwrite("assets/face1-detect-result.jpg", &image, &Vector::default())?;

    Ok(())
}
