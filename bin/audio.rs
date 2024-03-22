use mediainfo::stream;

fn main() {
    let mut data = stream::audio::AudioStream::default().set_id(2);
    data = data.set_format("test".to_string());
    println!("{:?}", data);
}

