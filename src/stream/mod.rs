pub mod audio;
pub mod subtitle;
pub mod video;

use crate::stream::audio::AudioStream;
use crate::stream::subtitle::SubtitleStream;
use crate::stream::video::VideoStream;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct MediaFile {
    pub filename: String,
    pub filepath: String,
    pub filesize: u64,
    pub filesize_is: String,
    pub filesize_iec: String,
    pub created_time: String,
    pub modified_time: String,
    pub duration: u64,
    pub format: String,
    pub writing_lib: String,
    pub bit_rate: u64,
    pub video_streams: Vec<VideoStream>,
    pub audio_streams: Vec<AudioStream>,
    pub subtitle_streams: Vec<SubtitleStream>,
}

impl MediaFile {
    pub fn set_filename(mut self, filename: String) -> Self {
        self.filename = filename;
        self
    }

    pub fn set_filepath(mut self, filepath: String) -> Self {
        self.filepath = filepath;
        self
    }

    pub fn set_filesize(mut self, filesize: u64) -> Self {
        self.filesize = filesize;
        self
    }

    pub fn set_filesize_is(mut self, filesize_is: String) -> Self {
        self.filesize_is = filesize_is;
        self
    }

    pub fn set_filesize_iec(mut self, filesize_iec: String) -> Self {
        self.filesize_iec = filesize_iec;
        self
    }

    pub fn set_created_time(mut self, created_time: String) -> Self {
        self.created_time = created_time;
        self
    }

    pub fn set_modified_time(mut self, modified_time: String) -> Self {
        self.modified_time = modified_time;
        self
    }

    pub fn set_duration(mut self, duration: u64) -> Self {
        self.duration = duration;
        self
    }

    pub fn set_format(mut self, format: String) -> Self {
        self.format = format;
        self
    }

    pub fn set_writing_lib(mut self, writing_lib: String) -> Self {
        self.writing_lib = writing_lib;
        self
    }

    pub fn set_bit_rate(mut self, bit_rate: u64) -> Self {
        self.bit_rate = bit_rate;
        self
    }

    pub fn set_video_streams(mut self, video_streams: Vec<VideoStream>) -> Self {
        self.video_streams = video_streams;
        self
    }

    pub fn set_audio_streams(mut self, audio_streams: Vec<AudioStream>) -> Self {
        self.audio_streams = audio_streams;
        self
    }

    pub fn set_subtitle_streams(mut self, subtitle_streams: Vec<SubtitleStream>) -> Self {
        self.subtitle_streams = subtitle_streams;
        self
    }
}
