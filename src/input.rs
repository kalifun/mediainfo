use crate::ffmpeg::format::context::common::StreamIter;
use crate::ffmpeg::format::context::Input;
use crate::format::EXTENSION_MAP;
use crate::format::FORMAT_MAP;
use std::path::Path;

pub struct FileInput<'a> {
    input: Input,
    path: &'a Path,
}

impl<'a> FileInput<'a> {
    pub fn new(input: Input, path: &'a Path) -> FileInput<'a> {
        FileInput { input, path }
    }

    pub fn format_name(&self) -> String {
        let format = self.input.format();
        if let Some(full_name) = FORMAT_MAP.get(format.name()) {
            full_name.to_string()
        } else {
            let extension = self
                .path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or_default()
                .to_lowercase();

            if let Some(full_name) = EXTENSION_MAP.get(extension.as_str()) {
                full_name.to_string()
            } else {
                format.name().to_string()
            }
        }
    }

    pub fn duration(&self) -> u64 {
        if self.input.duration() >= 0 {
            self.input.duration() as u64 / ffmpeg_next::ffi::AV_TIME_BASE as u64
        } else {
            0
        }
    }

    #[warn(dead_code)]
    pub fn get_bit_rate(&self) -> u64 {
        self.input.bit_rate() as u64
    }

    pub fn get_writing_lib(&self) -> String {
        let metadata = self.input.metadata();
        match metadata.get("encoder") {
            Some(name) => name.to_string(),
            None => "".to_string(),
        }
    }

    pub fn get_streams(&self) -> StreamIter {
        self.input.streams()
    }
}
