use opencv::{
    core::{Mat, Ptr, ToInputArray, ToOutputArray},
    objdetect::{FaceRecognizerSF, FaceRecognizerSF_DisType},
    prelude::{FaceRecognizerSFTrait, FaceRecognizerSFTraitConst},
};

pub enum FaceRecognizerSFThreshold {
    Cosine(f64), // 0.363
    Norm12(f64), // 1.128
}

pub struct Sface {
    mode: Ptr<FaceRecognizerSF>,
    threshold: FaceRecognizerSFThreshold,
}

impl Sface {
    pub fn new(path: &str) -> Result<Self, anyhow::Error> {
        let mode = FaceRecognizerSF::create(path, "", 0, 0)?;
        Ok(Self {
            mode,
            threshold: FaceRecognizerSFThreshold::Cosine(0.363),
        })
    }
    pub fn set_threshold(&mut self, threshold: FaceRecognizerSFThreshold) {
        self.threshold = threshold;
    }

    fn feature(
        &mut self,
        src_img: &impl ToInputArray,
        face_box: Option<&impl ToInputArray>,
        face_feature: &mut impl ToOutputArray,
    ) -> Result<(), anyhow::Error> {
        // preprocess
        if let Some(face_box) = face_box {
            let mut aligned_img_backend = Mat::default();
            self.mode
                .align_crop(src_img, face_box, &mut aligned_img_backend)?;
            self.mode.feature(&aligned_img_backend, face_feature)?;
        } else {
            self.mode.feature(src_img, face_feature)?;
        }
        Ok(())
    }

    pub fn r#match(
        &mut self,
        img1: &impl ToInputArray,
        face1: &impl ToInputArray,
        img2: &impl ToInputArray,
        face2: &impl ToInputArray,
    ) -> Result<MatchResult, anyhow::Error> {
        let mut feature1 = Mat::default();
        let mut feature2 = Mat::default();
        self.feature(img1, Some(face1), &mut feature1)?;
        self.feature(img2, Some(face2), &mut feature2)?;

        let result = match self.threshold {
            FaceRecognizerSFThreshold::Cosine(threshold) => {
                let distance = self.mode.match_(
                    &feature1,
                    &feature2,
                    FaceRecognizerSF_DisType::FR_COSINE.into(),
                )?;
                MatchResult {
                    distance,
                    is_matched: distance >= threshold,
                }
            }
            FaceRecognizerSFThreshold::Norm12(threshold) => {
                let distance = self.mode.match_(
                    &feature1,
                    &feature2,
                    FaceRecognizerSF_DisType::FR_NORM_L2.into(),
                )?;

                MatchResult {
                    distance,
                    is_matched: distance <= threshold,
                }
            }
        };
        Ok(result)
    }
}

#[derive(Debug)]
pub struct MatchResult {
    pub distance: f64,
    pub is_matched: bool,
}
