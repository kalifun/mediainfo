#[derive(Debug)]
pub struct SubtitleStream {
    pub id: usize,
    pub format: String,
    pub codec_info: String,
    pub duration: f64,
    pub bitrate:u64,
    pub frame_rate: f32,
    pub count_of_elements: u64,
    pub stream_size: u64,
    pub title: String,
    pub language: String,
    pub default: bool,
    pub forced: bool,
}
