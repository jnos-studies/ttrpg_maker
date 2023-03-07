use libtext;
use std::path::Path;

fn main(){
    let record_path = Path::new("test.wav");
    libtext::record_audio(record_path).unwrap();
    let test = libtext::transcribe_audio_file("test.wav");    
    println!("Audio file transcribed: {}", test.text);
}


