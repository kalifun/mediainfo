use mediainfo::read_file;

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    read_file(&path);
}
