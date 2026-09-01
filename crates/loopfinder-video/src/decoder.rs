use std::path::Path;
use thiserror::Error;

use crate::{frame::VideoFrame, metadata::VideoMetadata};

pub struct VideoDecoder {
    // ffmpeg state
    input: ffmpeg_next::format::context::Input,
    scaling_index: usize,
    decoder: ffmpeg_next::decoder::Video,
    scaler: ffmpeg_next::software::scaling::Context,
    time_base: ffmpeg_next::Rational,
}

#[derive(Error, Debug)]
pub enum DecoderError {
    #[error("ffmpeg decoder error")]
    Decode(#[from] ffmpeg_next::Error),
    #[error("no ffmpeg streams")]
    NoStreams,
}

impl VideoDecoder {
    pub fn open<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Self, DecoderError> {
        ffmpeg_next::init()?;

        let input = ffmpeg_next::format::input(path)?;

        let stream = input
            .streams()
            .best(ffmpeg_next::media::Type::Video)
            .ok_or(DecoderError::NoStreams)?;

        let stream_index = stream.index();
        let time_base = stream.time_base();

        let context = ffmpeg_next::codec::context::Context::from_parameters(stream.parameters())?;

        let decoder = context.decoder().video()?;

        let scaler = ffmpeg_next::software::scaling::Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            ffmpeg_next::format::Pixel::GRAY8,
            decoder.width(),
            decoder.height(),
            ffmpeg_next::software::scaling::Flags::BILINEAR,
        )?;

        let mut frame_index = 0;

        Ok(VideoDecoder {
            input,
            decoder,
            scaler,
            time_base,
            scaling_index: 0,
        })
    }

    pub fn metadata(&self) -> VideoMetadata {
        VideoMetadata::default()
    }

    pub fn next_frame(&mut self) -> Option<VideoFrame> {
        None
    }
}
