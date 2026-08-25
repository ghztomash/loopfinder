pub struct VideoFrame {
    pub timestamp: f64,
    pub width: u32,
    pub height: u32,
    /// Grayscale, one byte per pixel,
    pub data: Vec<u8>,
}
