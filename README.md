# mediainfo

A Rust library for parsing media container formats and extracting metadata and stream information. This library uses `ffmpeg-next` as a backend to extract information from media files.

## Features

*   Extracts general media information (format, duration, bitrate, etc.).
*   Identifies and extracts metadata for video, audio, and subtitle streams.
*   Provides detailed information for each stream, such as codec, resolution, frame rate, etc.
*   Calculates file size in both SI and IEC units.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
mediainfo = { git = "https://github.com/kalifun/mediainfo" }
```

Then, you can use the `read_file` function to extract media information:

```rust
use mediainfo::read_file;

fn main() {
    match read_file(&"path/to/your/media/file.mp4") {
        Ok(media_file) => {
            println!("{:#?}", media_file);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

## Run Examples

This crate includes two binary examples, `readfile` and `audio`. You can run them using `cargo run`:

### readfile

The `readfile` example demonstrates how to use the `read_file` function and prints the full `MediaFile` struct.

```bash
cargo run --bin readfile -- "path/to/your/media/file.mp4"
```

### audio

The `audio` example specifically extracts and prints information about the audio streams in a media file.

```bash
cargo run --bin audio -- "path/to/your/media/file.mp4"
```
