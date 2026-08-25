use crate::{candidate::LoopCandidate, frame::FrameFeature};

pub struct DetectorConfig {
    pub min_duration: f64,
    pub max_duration: f64,
    pub sequence_length: usize,
}

pub struct LoopDetecor {}

impl LoopDetecor {
    pub fn new(config: DetectorConfig) -> LoopDetecor {
        LoopDetecor {}
    }

    pub fn detect(frames: &[FrameFeature]) -> Vec<LoopCandidate> {
        let candidates = Vec::new();

        candidates
    }
}
