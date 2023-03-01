#![allow(clippy::uninlined_format_args)]

use hound::{SampleFormat, WavReader, Sample};
use std::path::Path;
use std::error::Error;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

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

fn convert_to_correct_sample_rate(path: &Path) -> Result<(), Box<dyn Error>> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let path_to_string = path.to_string_lossy();
    let path_len = path_to_string.len();

    let output_path = format!("{}_output.wav", &path_to_string[..path_len - 4]);

    let mut writer = hound::WavWriter::create(output_path, spec)?;
    
    for sample in reader.samples::<i32>() {
        let sample = sample.unwrap();
        let sample_16 = sample.as_i16();
        writer.write_sample(sample_16).unwrap();
    }
    Ok(())
}

fn main() {
    convert_to_correct_sample_rate(Path::new("untitled.wav")).expect("Unable to convert .wav file"); 
    let audio_path = Path::new("untitled_output.wav");
    let whisper_path = Path::new("whisper.cpp/models/ggml-base.en.bin");
    
    let original_samples = parse_wav_file(audio_path);
    let samples = whisper_rs::convert_integer_to_float_audio(&original_samples);

    let mut ctx =
        WhisperContext::new(&whisper_path.to_string_lossy()).expect("failed to open model");
    let params = FullParams::new(SamplingStrategy::default());
    ctx.full(params, &samples)
        .expect("failed to convert samples");

    let num_segments = ctx.full_n_segments();

    println!("{}", num_segments);

    for i in 0..num_segments {
        let segment = ctx.full_get_segment_text(i).expect("failed to get segment");
        let start_timestamp = ctx.full_get_segment_t0(i);
        let end_timestamp = ctx.full_get_segment_t1(i);
        println!("[{} - {}]: {}", start_timestamp, end_timestamp, segment);
    }
}
