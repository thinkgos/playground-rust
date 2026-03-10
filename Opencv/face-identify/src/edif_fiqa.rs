use std::ops::{Deref, DerefMut};

use opencv::{
    core::{self, Mat, Scalar, Size, ToInputArray},
    dnn::{self, NetTrait, read_net_from_onnx},
    imgproc,
};

pub struct EdifFiaq {
    mode: dnn::Net,
    input_size: Size,
}

impl EdifFiaq {
    pub fn new(onnx_file: &str) -> Result<Self, anyhow::Error> {
        let mode = read_net_from_onnx(onnx_file)?;

        Ok(Self {
            mode,
            input_size: Size::new(112, 112),
        })
    }
    pub fn set_input_size(&mut self, size: Size) -> &Self {
        self.input_size = size;
        self
    }
    pub fn infer(&mut self, img: &impl ToInputArray) -> Result<Mat, anyhow::Error> {
        // preprocess image
        let img_blob = self.preprocess(img)?;
        // forward
        self.mode.set_input_def(&img_blob)?;
        let quality_score = self.mode.forward_single_def()?;
        Ok(quality_score)
    }
    pub fn preprocess(&self, img: &impl ToInputArray) -> Result<Mat, anyhow::Error> {
        let mut img_rgb = Mat::default();
        imgproc::cvt_color(img, &mut img_rgb, imgproc::COLOR_BGR2RGB, 0)?;
        let mut img_resized = Mat::default();
        imgproc::resize_def(&img_rgb, &mut img_resized, self.input_size)?;
        // Scale to [0, 1] and normalize by mean=0.5, std=0.5
        // 归一化处理：((x / 255) - 0.5) / 0.5
        // 简化数学逻辑为：x * (1/127.5) - 1.0
        // OpenCV 的 blob_from_image 可以直接完成：[Resize] + [SwapRB] + [Mean Subtraction] + [Scaling]
        // 原图片的形状: (H, W, C) --> (N, C, H, W) N为数量(Batch)
        // 主要原因, 深度学习不接收单张图片, 接收的是一个批次(Batch), 大部份模型使用NCHW的布局在进行卷积运算, 内存访问更友好, 能更好利用SIMD指令集进行并行计算.
        let scale = 1.0 / 127.5;
        let mean = Scalar::new(127.5, 127.5, 127.5, 0.0); // 减去均值 (0.5 * 255)
        let img_blob = dnn::blob_from_image(
            &img_resized,
            scale,
            self.input_size,
            mean,
            false,
            false,
            core::CV_32F,
        )?;
        Ok(img_blob)
    }
}

impl Deref for EdifFiaq {
    type Target = dnn::Net;

    fn deref(&self) -> &Self::Target {
        &self.mode
    }
}

impl DerefMut for EdifFiaq {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mode
    }
}
