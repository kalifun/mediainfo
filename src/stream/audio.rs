#[derive(Default, Debug, Serialize, Deserialize)]
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
    pub fn set_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub fn set_format(mut self, format: String) -> Self {
        self.format = format;
        self
    }

    pub fn set_commercial_name(mut self, commercial_name: String) -> Self {
        self.commercial_name = commercial_name;
        self
    }

    pub fn set_codec(mut self, codec: String) -> Self {
        self.codec = codec;
        self
    }

    pub fn set_duration(mut self, duration: f64) -> Self {
        self.duration = duration;
        self
    }

    pub fn set_bitrate(mut self, bitrate: u64) -> Self {
        self.bitrate = bitrate;
        self
    }

    pub fn set_channel(mut self, channel: u32) -> Self {
        self.channel = channel;
        self
    }

    pub fn set_sample_rate(mut self, sample_rate: u32) -> Self {
        self.sample_rate = sample_rate;
        self
    }

    pub fn set_frame_rate(mut self, frame_rate: f64) -> Self {
        self.frame_rate = frame_rate;
        self
    }

    pub fn set_stream_size(mut self, stream_size: u64) -> Self {
        self.stream_size = stream_size;
        self
    }

    pub fn set_language(mut self, language: String) -> Self {
        self.language = language;
        self
    }

    pub fn set_default(mut self, default: bool) -> Self {
        self.default = default;
        self
    }

    pub fn set_forced(mut self, forced: bool) -> Self {
        self.forced = forced;
        self
    }
}
