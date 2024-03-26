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
}
