#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SubtitleStream {
    pub id: usize,
    pub format: String,
    pub codec_info: String,
    pub duration: f64,
    pub bitrate: u64,
    pub frame_rate: f32,
    pub count_of_elements: u64,
    pub stream_size: u64,
    pub title: String,
    pub language: String,
    pub default: bool,
    pub forced: bool,
}

impl SubtitleStream {
    pub fn set_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub fn set_format(mut self, format: String) -> Self {
        self.format = format;
        self
    }

    pub fn set_codec_info(mut self, codec_info: String) -> Self {
        self.codec_info = codec_info;
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

    pub fn set_frame_rate(mut self, frame_rate: f32) -> Self {
        self.frame_rate = frame_rate;
        self
    }

    pub fn set_count_of_elements(mut self, count_of_elements: u64) -> Self {
        self.count_of_elements = count_of_elements;
        self
    }

    pub fn set_stream_size(mut self, stream_size: u64) -> Self {
        self.stream_size = stream_size;
        self
    }

    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;
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
