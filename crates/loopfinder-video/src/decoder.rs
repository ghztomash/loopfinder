use std::path::Path;
use thiserror::Error;

use crate::{frame::VideoFrame, metadata::VideoMetadata};

pub struct VideoDecoder {
    // ffmpeg state
    input: ffmpeg_next::format::context::Input,
    stream_index: usize,
    decoder: ffmpeg_next::decoder::Video,
    scaler: ffmpeg_next::software::scaling::Context,
    time_base: ffmpeg_next::Rational,
    metadata: VideoMetadata,
    frame_index: usize,
    eof_sent: bool,
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
        let stream_duration = stream.duration();

        let context = ffmpeg_next::codec::context::Context::from_parameters(stream.parameters())?;
        let decoder = context.decoder().video()?;

        let duration = if stream_duration == ffmpeg_next::ffi::AV_NOPTS_VALUE {
            input.duration() as f64 / f64::from(ffmpeg_next::ffi::AV_TIME_BASE)
        } else {
            stream_duration as f64 * f64::from(time_base)
        };

        let frame_rate = decoder.frame_rate().map(f64::from).unwrap_or(0.0);

        let metadata = VideoMetadata {
            width: decoder.width(),
            height: decoder.height(),
            duration,
            frame_rate,
        };

        let scaler = ffmpeg_next::software::scaling::Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            ffmpeg_next::format::Pixel::GRAY8,
            decoder.width() / 4,
            decoder.height() / 4,
            ffmpeg_next::software::scaling::Flags::FAST_BILINEAR,
        )?;

        Ok(VideoDecoder {
            input,
            decoder,
            scaler,
            time_base,
            stream_index,
            metadata,
            frame_index: 0,
            eof_sent: false,
        })
    }

    pub fn metadata(&self) -> VideoMetadata {
        self.metadata.clone()
    }

    pub fn next_frame(&mut self) -> Option<VideoFrame> {
        let mut decoded = ffmpeg_next::frame::Video::empty();

        loop {
            if self.decoder.receive_frame(&mut decoded).is_ok() {
                let frame = self.scale_frame(&decoded);
                self.frame_index = self.frame_index.saturating_add(1);
                return Some(frame);
            }

            if self.eof_sent {
                return None;
            }

            match self
                .input
                .packets()
                .find(|(stream, _)| stream.index() == self.stream_index)
            {
                Some((_, packet)) => {
                    let _ = self.decoder.send_packet(&packet);
                }
                None => {
                    self.eof_sent = true;
                    let _ = self.decoder.send_eof();
                }
            }
        }
    }

    fn scale_frame(&mut self, decoded: &ffmpeg_next::frame::Video) -> VideoFrame {
        let mut scaled = ffmpeg_next::frame::Video::empty();
        self.scaler
            .run(decoded, &mut scaled)
            .expect("scaler input format/size must match decoder output");

        let width = scaled.width();
        let height = scaled.height();
        let stride = scaled.stride(0);
        let plane = scaled.data(0);

        let mut data = Vec::with_capacity((width * height) as usize);
        for row in 0..height as usize {
            let start = row * stride;
            data.extend_from_slice(&plane[start..start + width as usize]);
        }

        let timestamp = decoded
            .timestamp()
            .map(|pts| pts as f64 * f64::from(self.time_base))
            .unwrap_or(0.0);

        VideoFrame {
            timestamp,
            width,
            height,
            data,
        }
    }
}
