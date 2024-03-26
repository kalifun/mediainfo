mod format;
mod input;
pub mod stream;
mod utils;
#[macro_use]
extern crate serde;
use std::{fs, io, path::Path};
#[macro_use]
extern crate lazy_static;
use ffmpeg_next as ffmpeg;
use stream::MediaFile;

use crate::input::FileInput;
pub fn read_file<P: AsRef<Path>>(path: &P) -> io::Result<MediaFile> {
    let path = path.as_ref();
    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
    let file_size = fs::metadata(path).unwrap().len();
    let file_size_is = utils::convert_to_si_units(file_size);
    let file_size_iec = utils::convert_to_iec_units(file_size);
    println!(
        "file_name: {} \n file_size: {} \n file_size_is {} \n file_size_iec {}",
        file_name, file_size, file_size_is, file_size_iec,
    );
    let format_ctx = ffmpeg::format::input(&path)?;

    let fi = FileInput::new(format_ctx, path);
    let full_format_name = fi.format_name();
    println!("{}", full_format_name);

    let media_file = MediaFile::default();
    Ok(media_file)
}
