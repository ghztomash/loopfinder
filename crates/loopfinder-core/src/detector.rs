use crate::{candidate::LoopCandidate, frame::FrameFeature};

pub struct DetectorConfig {
    pub min_duration: f64,
    pub max_duration: f64,
    pub sequence_length: usize,
}

pub enum DetectorError {}

pub struct LoopDetecor {}

impl LoopDetecor {
    pub fn new(config: DetectorConfig) -> LoopDetecor {
        LoopDetecor {}
    }

    pub fn detect(&self, frames: &[FrameFeature]) -> Result<Vec<LoopCandidate>, DetectorError> {
        let candidates = Vec::new();

        Ok(candidates)
    }
}
