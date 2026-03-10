use opencv::{
    core::{Point, Point2i, Rect2i, Scalar, ToInputOutputArray},
    imgproc,
};

use crate::yu_net::Detection;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct DetectionVisualizeColor {
    pub face: Scalar,
    pub eye_right: Scalar,
    pub eye_left: Scalar,
    pub nose: Scalar,
    pub corner_right: Scalar,
    pub corner_left: Scalar,
    pub score: Scalar,
}

impl DetectionVisualizeColor {
    pub fn new(&self) -> Self {
        Self::default()
    }
}

impl Default for DetectionVisualizeColor {
    fn default() -> Self {
        Self {
            face: Scalar::new(0.0, 255.0, 0.0, 0.0),           // green
            eye_right: Scalar::new(255.0, 0.0, 0.0, 0.0),      // blue
            eye_left: Scalar::new(0.0, 0.0, 255.0, 0.0),       // red
            nose: Scalar::new(0.0, 255.0, 0.0, 0.0),           // green
            corner_right: Scalar::new(255.0, 0.0, 255.0, 0.0), // magenta
            corner_left: Scalar::new(0.0, 255.0, 255.0, 0.0),  // yellow
            score: Scalar::new(0.0, 0.0, 255.0, 0.0),          // blue
        }
    }
}

impl DetectionVisualizeColor {
    pub fn draw(
        &self,
        img: &mut impl ToInputOutputArray,
        detect: &Detection,
    ) -> Result<(), anyhow::Error> {
        self.draw_box_face(img, detect.face.to::<i32>().unwrap_or_default())?;
        self.draw_score(
            img,
            Point2i::new(detect.face.x as i32, detect.face.y as i32 + 12),
            detect.score,
        )?;
        imgproc::circle(
            img,
            detect.eye_right.to::<i32>().unwrap_or_default(),
            2,
            self.eye_right,
            2,
            imgproc::LINE_8,
            0,
        )?;
        imgproc::circle(
            img,
            detect.eye_left.to::<i32>().unwrap_or_default(),
            2,
            self.eye_left,
            2,
            imgproc::LINE_8,
            0,
        )?;
        imgproc::circle(
            img,
            detect.nose.to::<i32>().unwrap_or_default(),
            2,
            self.nose,
            2,
            imgproc::LINE_8,
            0,
        )?;
        imgproc::circle(
            img,
            detect.corner_right.to::<i32>().unwrap_or_default(),
            2,
            self.corner_right,
            2,
            imgproc::LINE_8,
            0,
        )?;
        imgproc::circle(
            img,
            detect.corner_left.to::<i32>().unwrap_or_default(),
            2,
            self.corner_left,
            2,
            imgproc::LINE_8,
            0,
        )?;
        Ok(())
    }
    pub fn draw_box_face(
        &self,
        img: &mut impl ToInputOutputArray,
        face: Rect2i,
    ) -> Result<(), anyhow::Error> {
        imgproc::rectangle_def(img, face, self.face)?;
        Ok(())
    }
    pub fn draw_score(
        &self,
        img: &mut impl ToInputOutputArray,
        org: Point2i,
        score: f32,
    ) -> Result<(), anyhow::Error> {
        imgproc::put_text_def(
            img,
            &format!("{:.4}", score),
            org,
            imgproc::FONT_HERSHEY_SIMPLEX,
            0.5,
            self.score,
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct RecognitionVisualizeColor {
    pub matched_face: Scalar,
    pub mismatched_face: Scalar,
}

#[derive(Debug)]
pub struct MatchResult {
    pub detect: Detection,
    pub distance: f64,
    pub is_matched: bool,
}

impl RecognitionVisualizeColor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn draw(
        &self,
        img: &mut impl ToInputOutputArray,
        results: &[MatchResult],
    ) -> Result<(), anyhow::Error> {
        for result in results {
            let face = result.detect.face.to::<i32>().unwrap_or_default();
            let color = if result.is_matched {
                self.matched_face
            } else {
                self.mismatched_face
            };
            imgproc::rectangle_def(img, face, color)?;
            imgproc::put_text_def(
                img,
                &format!("{:.4}", result.distance),
                Point::new(result.detect.face.x as i32, result.detect.face.y as i32 - 5),
                imgproc::FONT_HERSHEY_DUPLEX,
                0.4,
                color,
            )?;
        }
        Ok(())
    }
    pub fn draw_box_matched(
        &self,
        img: &mut impl ToInputOutputArray,
        face: Rect2i,
    ) -> Result<(), anyhow::Error> {
        imgproc::rectangle_def(img, face, self.matched_face)?;
        Ok(())
    }
    pub fn draw_box_mismatched(
        &self,
        img: &mut impl ToInputOutputArray,
        face: Rect2i,
    ) -> Result<(), anyhow::Error> {
        imgproc::rectangle_def(img, face, self.mismatched_face)?;
        Ok(())
    }
}

impl Default for RecognitionVisualizeColor {
    fn default() -> Self {
        Self {
            matched_face: Scalar::new(0.0, 255.0, 0.0, 0.0), // green
            mismatched_face: Scalar::new(0.0, 0.0, 255.0, 0.0), // red
        }
    }
}
