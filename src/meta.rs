use ffmpeg_next as ffmpeg;
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
                media::Type::Audio => audios.push(parse_audio(&stream)),
                media::Type::Video => videos.push(parse_video(&stream)),
                media::Type::Subtitle => subtitles.push(parse_subtitle(&stream)),
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

fn parse_video(stream: &ffmpeg::format::stream::Stream) -> VideoStream {
    use crate::stream::video::VideoStream;
    let params = stream.parameters();
    let codec_ctx = match ffmpeg::codec::context::Context::from_parameters(params) {
        Ok(ctx) => ctx,
        Err(_) => return VideoStream::default(),
    };
    let codec_id = format!("{:?}", codec_ctx.id());
    let duration = stream.duration() as f64 * f64::from(stream.time_base());
    let decoder = match codec_ctx.decoder().video() {
        Ok(dec) => dec,
        Err(_) => return VideoStream::default(),
    };
    let pixel_format = decoder.format();
    let frame_rate = decoder.frame_rate().map(|r| r.0 as f32 / r.1 as f32).unwrap_or(0.0);
    let aspect = decoder.aspect_ratio();
    let aspect_str = format!("{}:{}", aspect.numerator(), aspect.denominator());
    let color_space = format!("{:?}", decoder.color_space());
    let resolution = (decoder.width(), decoder.height());
    let bit_depth = pixel_format.descriptor().map(|desc| unsafe {
        let ptr = desc.as_ptr();
        if ptr.is_null() {
            return 0;
        }
        let nb_components = (*ptr).nb_components as usize;
        let mut max_depth = 0;
        for i in 0..nb_components {
            let comp = (*ptr).comp[i];
            let depth = comp.depth as u32;
            if depth > max_depth {
                max_depth = depth;
            }
        }
        max_depth
    }).unwrap_or(0);
    VideoStream::default()
        .set_codec(codec_id)
        .set_format(format!("{:?}", pixel_format))
        .set_frame_rate(frame_rate)
        .set_resolution(resolution)
        .set_display_aspect_ratio(aspect_str)
        .set_bitrate(decoder.bit_rate() as u64)
        .set_duration(duration)
        .set_color_space(color_space)
        .set_bit_depth(bit_depth)
        .set_language(stream.metadata().get("language").unwrap_or("").to_string())
}

fn parse_audio(stream: &ffmpeg::format::stream::Stream) -> AudioStream {
    use crate::stream::audio::AudioStream;
    let params = stream.parameters();
    let codec_ctx = match ffmpeg::codec::context::Context::from_parameters(params) {
        Ok(ctx) => ctx,
        Err(_) => return AudioStream::default(),
    };
    let duration = stream.duration() as f64 * f64::from(stream.time_base());
    let decoder = match codec_ctx.decoder().audio() {
        Ok(dec) => dec,
        Err(_) => return AudioStream::default(),
    };
    let codec_id = format!("{:?}", decoder.id());
    let sample_rate = decoder.rate() as u32;
    let channels = decoder.channels() as u32;
    let bitrate = decoder.bit_rate() as u64;
    AudioStream::default()
        .set_id(stream.index())
        .set_format(codec_id.clone())
        .set_commercial_name(String::new())
        .set_codec(codec_id)
        .set_duration(duration)
        .set_bitrate(bitrate)
        .set_channel(channels)
        .set_sample_rate(sample_rate)
        .set_frame_rate(0.0)
        .set_language(stream.metadata().get("language").unwrap_or("").to_string())
}

fn parse_subtitle(stream: &ffmpeg::format::stream::Stream) -> SubtitleStream {
    use crate::stream::subtitle::SubtitleStream;
    let params = stream.parameters();
    let codec_ctx = match ffmpeg::codec::context::Context::from_parameters(params) {
        Ok(ctx) => ctx,
        Err(_) => return SubtitleStream::default(),
    };
    let duration = stream.duration() as f64 * f64::from(stream.time_base());
    SubtitleStream::default()
        .set_id(stream.index())
        .set_format(format!("{:?}", codec_ctx.id()))
        .set_codec_info(format!("{:?}", codec_ctx.id()))
        .set_duration(duration)
        .set_language(stream.metadata().get("language").unwrap_or("").to_string())
}
