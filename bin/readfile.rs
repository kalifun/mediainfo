use mediainfo::read_file;

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    if let Err(e) = read_file(&path) {
        eprintln!("Error: {}", e);
    }
}
