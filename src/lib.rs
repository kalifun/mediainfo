mod format;
mod input;
mod meta;
pub mod stream;
mod utils;
#[macro_use]
extern crate serde;
use std::{fs, io, path::Path};
#[macro_use]
extern crate lazy_static;
use crate::input::FileInput;
use ffmpeg_next as ffmpeg;
use stream::MediaFile;

pub fn read_file<P: AsRef<Path>>(path: &P) -> io::Result<MediaFile> {
    let path = path.as_ref();
    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default()
        .to_string();

    let metadata = fs::metadata(path)?;
    let file_size = metadata.len();
    let file_size_is = utils::convert_to_si_units(file_size);
    let file_size_iec = utils::convert_to_iec_units(file_size);

    let format_ctx = ffmpeg::format::input(&path)?;
    let fi = FileInput::new(format_ctx, path);
    let stream_meta = meta::StreamMeta::new(fi.get_streams());

    let media_file = MediaFile::default()
        .set_filename(file_name)
        .set_filepath(path.to_string_lossy().to_string())
        .set_filesize(file_size)
        .set_filesize_is(file_size_is)
        .set_filesize_iec(file_size_iec)
        .set_format(fi.format_name())
        .set_writing_lib(fi.get_writing_lib())
        .set_duration(fi.duration())
        .set_video_streams(stream_meta.get_videos().clone())
        .set_audio_streams(stream_meta.get_audios().clone())
        .set_subtitle_streams(stream_meta.get_subtitles().clone());

    println!("{:?}", media_file);
    Ok(media_file)
}
