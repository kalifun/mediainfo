#[derive(Default, Debug, Serialize, Deserialize)]
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

impl VideoStream {
    pub fn set_codec(mut self, codec: String) -> Self {
        self.codec = codec;
        self
    }

    pub fn set_format(mut self, format: String) -> Self {
        self.format = format;
        self
    }

    pub fn set_hdr_format(mut self, hdr_format: String) -> Self {
        self.hdr_format = hdr_format;
        self
    }

    pub fn set_frame_rate(mut self, frame_rate: f32) -> Self {
        self.frame_rate = frame_rate;
        self
    }

    pub fn set_resolution(mut self, resolution: (u32, u32)) -> Self {
        self.resolution = resolution;
        self
    }

    pub fn set_display_aspect_ratio(mut self, display_aspect_ratio: String) -> Self {
        self.display_aspect_ratio = display_aspect_ratio;
        self
    }

    pub fn set_bitrate(mut self, bitrate: u64) -> Self {
        self.bitrate = bitrate;
        self
    }

    pub fn set_duration(mut self, duration: f64) -> Self {
        self.duration = duration;
        self
    }

    pub fn set_color_space(mut self, color_space: String) -> Self {
        self.color_space = color_space;
        self
    }

    pub fn set_bit_depth(mut self, bit_depth: u32) -> Self {
        self.bit_depth = bit_depth;
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

    pub fn set_color_primaries(mut self, color_primaries: String) -> Self {
        self.color_primaries = color_primaries;
        self
    }
}
