#[derive(Debug, Serialize, Deserialize)]
pub struct AudioStream {
    pub id: usize,
    pub format: String,
    pub commercial_name: String,
    pub codec: String,
    pub duration: f64,
    pub bitrate: u64,
    pub channel: u32,
    pub sample_rate: u32,
    pub frame_rate: f64,
    pub stream_size: u64,
    pub language: String,
    pub default: bool,
    pub forced: bool,
}


impl AudioStream {
    
}