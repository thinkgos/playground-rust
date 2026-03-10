use face_identify::{
    edif_fiqa::EdifFiaq,
    yu_net::{Detection, YuNet},
};
use opencv::{
    calib3d,
    core::{
        self, Mat, MatTraitConst, MatTraitConstManual, Point, Point2f, Scalar, Size, ToInputArray,
        Vector,
    },
    imgcodecs, imgproc,
    prelude::FaceDetectorYNTrait,
};

fn main() -> Result<(), anyhow::Error> {
    let mut quality = EdifFiaq::new("../model/ediffiqa_tiny_jun2024.onnx")?;
    let mut detector = YuNet::new("../model/face_detection_yunet_2023mar.onnx")?;

    let img = imgcodecs::imread("assets/face1.jpg", imgcodecs::IMREAD_COLOR)?;
    let size = img.size()?;

    detector.set_input_size(size)?;
    let (ret, faces) = detector.detect(&img)?;
    if ret == 0 {
        println!("No face detected.");
        return Ok(());
    } else {
        println!("Detected faces: {:?}", ret);
    }

    // 人脸对齐
    let aligned_image = align_image(&img, &faces)?;
    // 计算人脸质量
    let quality_result = quality.infer(&aligned_image)?;
    // 获取单值结果 (假设输出是 1x1 的 Mat)
    let quality: f32 = *quality_result.at_2d::<f32>(0, 0)?;

    let mut aligned_image_copy = aligned_image.clone();

    //  cv.putText(output, f"{results:.3f}", (0, 20), cv.FONT_HERSHEY_DUPLEX, .8, (0, 0, 255))

    imgproc::put_text_def(
        &mut aligned_image_copy,
        &format!("{:.3}", quality),
        Point::new(0, 20),
        imgproc::FONT_HERSHEY_DUPLEX,
        0.8,
        Scalar::new(0.0, 0.0, 255.0, 0.0),
    )?;

    imgcodecs::imwrite(
        "assets/face1-quality-result.jpg",
        &aligned_image_copy,
        &Vector::default(),
    )?;
    Ok(())
}

// 标准参考点, 基于112 * 112 基准.
const REFERENCE_FACIAL_POINTS: [Point2f; 5] = [
    Point2f::new(38.2946, 51.6963),   // 右眼
    Point2f::new(73.5318, 51.5014),   // 左眼
    Point2f::new(56.0252, 71.7366),   // 鼻尖
    Point2f::new(41.5493, 92.3655),   // 右嘴角
    Point2f::new(70.729904, 92.2041), // 左嘴角
];

/// 执行人脸对齐
pub fn align_image(image: &impl ToInputArray, faces: &Mat) -> Result<Mat, anyhow::Error> {
    // * 定义标准参考点(Reference Points)
    let ref_pts = Vector::<Point2f>::from_iter(REFERENCE_FACIAL_POINTS);

    // * 获取观测点(Source Points)
    // 从 YuNet 的检测数据中提取源关键点, YuNet 的输出通常是[N, 15], 其中 index 4-13 是 5 个关键点的(x, y)
    let row = faces.at_row::<f32>(0)?;
    let detection: Detection = row.try_into()?;
    let mut src_pts = Vector::<Point2f>::with_capacity(5);
    src_pts.push(detection.eye_right);
    src_pts.push(detection.eye_left);
    src_pts.push(detection.nose);
    src_pts.push(detection.corner_right);
    src_pts.push(detection.corner_left);

    // * 计算相似变换矩阵 (Estimate Affine Partial 2D)
    // 这里对应estimate_affine_partial_2d, 采用LMEDS算法
    // * 部分仿射变换(Partial Affine Transform), 它比全仿射变换多了一个约束, 即保持图像的横纵比不变,
    // * 它只允许旋转(Rotation)、平移(Translation)和等比例缩放(Scaling).
    // * 对找一个矩阵M, 使用src_pts * M ≈ = ref_pts 的误差最小.
    //
    let tfm = calib3d::estimate_affine_partial_2d(
        &src_pts, // 源点 (从图中检测到的)
        &ref_pts, // 目标点 (预定义的标准坐标)
        &mut core::no_array(),
        calib3d::LMEDS, // LMEDS算法, 鲁棒估计方法(最小中值平方法)
        3.0,            // ransac_reproj_threshold
        2000,           // max_iters
        0.99,           // confidence
        10,             // refineIters
    )?;

    // * 应用仿射变换 (Warp Affine)
    let mut face_img = Mat::default();
    imgproc::warp_affine(
        image,
        &mut face_img,
        &tfm,
        Size::new(112, 112),   // 输出固定大小 112x112
        imgproc::INTER_LINEAR, // 插值方式
        core::BORDER_CONSTANT, // 边界填充
        Scalar::default(),     // 填充颜色(黑色)
    )?;

    Ok(face_img)
}
