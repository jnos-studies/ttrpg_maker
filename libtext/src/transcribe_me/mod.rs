use std::path::Path;
use std::i16;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};
use narratives::TypedNarrative;
use hound::{SampleFormat, WavReader};
// Helper Functions for the transcribe_audio_file public function
fn parse_wav_file(path: &Path) -> Vec<i16> {
    let reader = WavReader::open(path).expect("failed to read file");

    if reader.spec().channels != 1 {
        panic!("expected mono audio file");
    }
    if reader.spec().sample_format != SampleFormat::Int {
        panic!("expected integer sample format");
    }
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


fn convert_sample_rate(file_name: &str) -> Result<(), hound::Error> {
    // Open the input WAV file
    let mut reader = hound::WavReader::open(file_name)?;
    assert_eq!(reader.spec().channels, 1);
    assert_eq!(reader.spec().sample_format, hound::SampleFormat::Int);

    // Set up the output WAV file with the new sample rate store in test_wavs for now
    let output_file_name = format!("test_wavs/{}_output.wav", Path::new(file_name).file_stem().unwrap().to_str().unwrap());
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(output_file_name, spec)?;

    // Read samples from the input file and write them to the output file
    // Convert to correct sampleby only writting every nth sample, where n
    // is the ratio of the input and output sample rates
    const N: usize = 5;
    let ratio = reader.spec().sample_rate as f32 / writer.spec().sample_rate as f32;
    for (i, sample) in reader.samples::<i16>().enumerate() {
        if i % N == 0 {
            let sample = sample.unwrap();
            let resampled_sample = (sample as f32 * ratio) as i16;
            writer.write_sample(resampled_sample).unwrap();
        }
    }
    Ok(())
}


pub fn transcribe_audio_file(audio_file: &str) -> TypedNarrative {
    let path_string = audio_file.to_string();
    let path_len = &path_string.len();
    let output_string = format!("{}_output.wav", &path_string[..path_len - 4]);

    convert_sample_rate(&audio_file).expect("unable to convert .wav file"); 
    
    let audio_output_path = Path::new(&output_string);
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

