use hound::{SampleFormat, WavReader};
use std::path::Path;
use std::error::Error;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};
use narratives::TypedNarrative;

fn parse_wav_file(path: &Path) -> Vec<i16> {
    let reader = WavReader::open(path).expect("failed to read file");

    if reader.spec().channels != 1 {
        panic!("expected mono audio file");
    }
    if reader.spec().sample_format != SampleFormat::Int {
        panic!("expected integer sample format");
    }
    println!("Sample rate: {}", reader.spec().sample_rate);
    if reader.spec().sample_rate != 16000 {
        panic!("expected 16KHz sample rate");
    }
    if reader.spec().bits_per_sample != 16 {
        panic!("expected 16 bits per sample");
    }

    reader
        .into_samples::<i16>()
        .map(|x| x.expect("sample"))
        .collect::<Vec<_>>()
}

// Converts mono i32 .wav audio into i16
fn convert_to_correct_sample_rate(path: &Path) -> Result<(), Box<dyn Error>> {
    let mut reader = hound::WavReader::open(path)?;

    let new_sample_rate = 16_000;
    // speed up after conversion to i16
    let ratio = 5.0;

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: new_sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let path_to_string = path.to_string_lossy();
    let path_len = path_to_string.len();

    let output_path = format!("{}_output.wav", &path_to_string[..path_len - 4]);

    let mut writer = hound::WavWriter::create(output_path, spec)?;
    
    for (i, sample) in reader.samples::<i16>().enumerate() {
        if i % 2 == 0 {
            let sample = sample.unwrap();
            let resampled_sample = (sample as f64 * ratio) as i16;
            writer.write_sample(resampled_sample).unwrap();
        }
    }

    drop(reader);
    writer.finalize().unwrap();
    Ok(())
}

pub fn transcribe_audio_file(audio_file: &str) -> TypedNarrative {
    convert_to_correct_sample_rate(Path::new(audio_file)).expect("Unable to convert .wav file"); 
    let path_len = audio_file.len();
    let audio_output_path = format!("{}_output.wav", &audio_file[..path_len - 4]);
    let audio_output_path = Path::new(&audio_output_path);
    let whisper_path = Path::new("whisper.cpp/models/ggml-base.en.bin");
    
    let original_samples = parse_wav_file(audio_output_path);
    let samples = whisper_rs::convert_integer_to_float_audio(&original_samples);

    let mut ctx =
        WhisperContext::new(&whisper_path.to_string_lossy()).expect("failed to open model");
    let params = FullParams::new(SamplingStrategy::default());
    ctx.full(params, &samples)
        .expect("failed to convert samples");

    let num_segments = ctx.full_n_segments();

    let mut result_text: String = String::from("");
    for i in 0..num_segments {
        let segment = ctx.full_get_segment_text(i).expect("failed to get segment");
        result_text += segment.as_str();
    }

    TypedNarrative::new(result_text)
}

