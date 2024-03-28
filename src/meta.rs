use crate::ffmpeg::media;

use crate::ffmpeg::format::context::common::StreamIter;

use crate::stream::audio::AudioStream;
use crate::stream::subtitle::SubtitleStream;
use crate::stream::video::VideoStream;

#[derive(Debug)]
pub struct StreamMeta {
    audios: Vec<AudioStream>,
    subtitles: Vec<SubtitleStream>,
    videos: Vec<VideoStream>,
}

impl StreamMeta {
    pub fn new(streams: StreamIter) -> StreamMeta {
        let mut audios = Vec::new();
        let mut subtitles = Vec::new();
        let mut videos = Vec::new();
        for stream in streams {
            let medium = stream.parameters().medium();
            match medium {
                media::Type::Audio => audios.push(parse_audio()),
                media::Type::Video => videos.push(parse_video()),
                media::Type::Subtitle => subtitles.push(parse_subtitle()),
                _ => {}
            }
        }
        StreamMeta {
            audios,
            subtitles,
            videos,
        }
    }

    pub fn get_audios(&self) -> &Vec<AudioStream> {
        &self.audios
    }

    pub fn get_subtitles(&self) -> &Vec<SubtitleStream> {
        &self.subtitles
    }

    pub fn get_videos(&self) -> &Vec<VideoStream> {
        &self.videos
    }
}

fn parse_video() -> VideoStream {
    VideoStream::default()
}

fn parse_audio() -> AudioStream {
    AudioStream::default()
}

fn parse_subtitle() -> SubtitleStream {
    SubtitleStream::default()
}
