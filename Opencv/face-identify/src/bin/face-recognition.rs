use face_identify::sface::Sface;
use face_identify::visualize::{MatchResult, RecognitionVisualizeColor};
use opencv::core::Vector;
use opencv::{imgcodecs, prelude::*};

use face_identify::yu_net::{Detection, YuNet};

fn main() -> Result<(), anyhow::Error> {
    let mut detector = YuNet::new("../model/face_detection_yunet_2023mar.onnx")?;
    let mut recognizer = Sface::new("../model/face_recognition_sface_2021dec.onnx")?;

    let face_target = imgcodecs::imread("assets/face1.jpg", imgcodecs::IMREAD_COLOR)?;
    let mut face_query = imgcodecs::imread("assets/face1.jpg", imgcodecs::IMREAD_COLOR)?;

    detector.set_input_size(face_target.size()?)?;
    let (ret1, face_target_result) = detector.detect(&face_target)?;
    if ret1 == 0 {
        println!("No target face detected.");
        return Ok(());
    }
    detector.set_input_size(face_query.size()?)?;
    let (ret2, face_query_results) = detector.detect(&face_query)?;
    if ret2 == 0 {
        println!("No query face detected.");
        return Ok(());
    }
    println!("Detected query faces: {:?}, target face: {:?}", ret1, ret2);

    let face_target_res1 = face_target_result.at_row::<f32>(0)?;
    // ? 需要去掉vec最后的score, 但没去掉好像也没报错?
    let face_target_res1 = Vector::from_slice(face_target_res1);

    let mut results = Vec::with_capacity(face_query_results.rows() as usize);
    for i in 0..face_query_results.rows() {
        let row = face_query_results.at_row::<f32>(i)?;
        let detection: Detection = row.try_into()?;
        let res = Vector::from_slice(row);

        let match_result =
            recognizer.r#match(&face_target, &res, &face_query, &face_target_res1)?;

        results.push(MatchResult {
            detect: detection,
            distance: match_result.distance,
            is_matched: match_result.is_matched,
        });
    }

    let rvc = RecognitionVisualizeColor::new();
    rvc.draw(&mut face_query, &results)?;

    opencv::imgcodecs::imwrite(
        "assets/face1-recognition-result.jpg",
        &face_query,
        &Vector::default(),
    )?;

    Ok(())
}
