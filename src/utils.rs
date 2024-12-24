use find_folder::Search;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn play_sound(file_path: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = File::open(file_path).expect("Failed to open audio file");
    let source = Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}

pub fn find_assets_folder() -> Result<PathBuf, String> {
    Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .map_err(|_| "Assets folder not found".to_string())
}
