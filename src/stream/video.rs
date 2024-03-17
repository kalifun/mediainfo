
#[derive(Debug)]
pub struct VideoStream {
    pub codec: String,
    pub format: String,
    pub hdr_format: String,
    pub frame_rate: f32,
    pub resolution: (u32, u32),
    pub display_aspect_ratio: String,
    pub bitrate: u64,
    pub duration: f64,
    pub color_space: String,
    pub bit_depth: u32,
    pub stream_size: u64,
    pub language: String,
    pub default: bool,
    pub forced: bool,
    pub color_primaries: String,
}
