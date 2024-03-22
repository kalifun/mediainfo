use crate::ffmpeg::format::Input;
use std::collections::HashMap;

use std::path::Path;
lazy_static! {
    static ref FORMAT_MAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("3g2", "3GP2 (3G2) Multimedia File Format");
        map.insert("3gp", "3GP (3GPP) Multimedia File Format");
        map.insert("aac", "Advanced Audio Coding (AAC)");
        map.insert("ac3", "AC-3 Audio");
        map.insert("amr", "Adaptive Multi-Rate (AMR) Audio");
        map.insert("ape", "Monkey's Audio");
        map.insert("asf", "Advanced Systems Format (ASF)");
        map.insert("au", "Sun AU (Au) Audio File");
        map.insert("avi", "Audio Video Interleaved (AVI)");
        map.insert("dv", "DV (Digital Video)");
        map.insert("flac", "Free Lossless Audio Codec (FLAC)");
        map.insert("flv", "Flash Video (FLV)");
        map.insert("gif", "Graphics Interchange Format (GIF)");
        map.insert("m4a", "MPEG-4 Audio (M4A)");
        map.insert("m4v", "MPEG-4 Video (M4V)");
        map.insert("mkv", "Matroska Multimedia Container (MKV)");
        map.insert("mov", "QuickTime File Format (MOV)");
        map.insert("mp3", "MPEG-1 Audio Layer III (MP3)");
        map.insert("mp4", "MPEG-4 Part 14 (MP4)");
        map.insert("mpeg", "MPEG-1 System Stream");
        map.insert("mpg", "MPEG-1 System Stream");
        map.insert("ogg", "Ogg Multimedia Container Format");
        map.insert("opus", "Opus Audio Codec");
        map.insert("rm", "RealMedia (RM)");
        map.insert("wav", "Waveform Audio File Format (WAV)");
        map.insert("webm", "Web Media (WebM)");
        map.insert("wma", "Windows Media Audio (WMA)");
        map.insert("wmv", "Windows Media Video (WMV)");

        map
    };
}

lazy_static! {
    static ref EXTENSION_MAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("mov", "QuickTime File Format");
        map.insert("mp4", "MPEG-4 Part 14");
        map.insert("m4a", "MPEG-4 Audio");
        map.insert("avi", "Audio Video Interleaved");
        map.insert("mkv", "Matroska Multimedia Container");
        map.insert("flv", "Flash Video");
        map.insert("webm", "Web Media");
        map.insert("wav", "Waveform Audio File Format");
        map.insert("mp3", "MPEG-1 Audio Layer III");
        map
    };
}

pub fn format_name(format: &Input, path: &Path) -> String {
    if let Some(full_name) = FORMAT_MAP.get(format.name()) {
        full_name.to_string()
    } else {
        let extension = path
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
