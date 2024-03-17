pub mod audio;
pub mod subtitle;
pub mod video;

use crate::stream::subtitle::SubtitleStream;
use crate::stream::audio::AudioStream;
use crate::stream::video::VideoStream;


#[derive(Debug)]
pub struct MediaFile {
    pub filename: String,
    pub filepath: String,
    pub filesize: u64,
    pub filesize_is: String,
    pub filesize_iec: String,
    pub created_time: String,
    pub modified_time: String,
    pub duration: f64,
    pub format: String,
    pub writing_lib: String,
    pub video_streams: Vec<VideoStream>,
    pub audio_streams: Vec<AudioStream>,
    pub subtitle_streams: Vec<SubtitleStream>,
}



