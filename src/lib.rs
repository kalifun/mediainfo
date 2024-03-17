mod utils;
mod stream;
#[macro_use]
extern crate serde;
use std::{fs, path::Path};

use ffmpeg_next as ffmpeg;


pub fn read_file<P: AsRef<Path>>(path: &P){
    let path = path.as_ref();
    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
    let file_size = fs::metadata(path).unwrap().len();
    let file_size_is = utils::convert_to_si_units(file_size);
    let file_size_iec = utils::convert_to_iec_units(file_size);
    println!(
        "file_name: {} \n file_size: {} \n file_size_is {} \n file_size_iec {}",
        file_name, file_size, file_size_is, file_size_iec,
    );
    let mut format_ctx = ffmpeg::format::input(&path);
    match format_ctx {
        Ok(input) => {
            println!("name {}",input.format().name());
        },
        Err(err) => {
            eprintln!("{}", err);
        }
    }
    
}