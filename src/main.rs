use libtext;

fn main(){
    let test = libtext::transcribe_audio_file("test.wav");    
    println!("Audio file transcribed: {}", test.text);
}


