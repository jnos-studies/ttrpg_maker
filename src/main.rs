use libtext;

fn main(){
    libtext::record_audio("test_wavs/test2.wav").expect("Recording Failure");
    let test = libtext::transcribe_audio_file("test_wavs/test2.wav");    
    println!("Audio file transcribed: {}", test.text);
}


