use libtext;

fn main(){
    libtext::record_audio().expect("Recording Failure");
    let test = libtext::transcribe_audio_file("recorded.wav");    
    println!("Audio file transcribed: {}", test.text);
}


