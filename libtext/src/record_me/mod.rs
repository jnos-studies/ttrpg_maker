use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample};
use hound::WavSpec;
use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};

pub fn record_audio(ui: &mut egui::Ui, file_name: &str) -> Result<(), anyhow::Error> {
    let host = cpal::default_host();

    // Set up the input device and stream with the default input config.
    let device = host.default_input_device();

    let config = device.as_ref()
        .unwrap()
        .default_input_config()
        .expect("Failed to get default input config");
    println!("Default input config: {:?}", config);

    // The WAV file we're recording to.
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44_100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int
    };

    let writer = hound::WavWriter::create(file_name, spec)?;
    let writer = Arc::new(Mutex::new(Some(writer)));

    // A flag to indicate that recording is in progress.
    println!("Begin recording...");

    // Run the input stream on a separate thread.
    let writer_2 = writer.clone();

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };

    let stream = device.unwrap().build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i16, i16>(data, &writer_2),
            err_fn,
            None,
        )?;

    stream.play()?;

    // A flag to indicate that recording is in progress.
    let mut is_recording = true;

    // Start the UI thread.
    let ctx = egui::Context::default();
    
    egui::Window::new("Recording").show(&ctx, |ui| {
        if ui.add(egui::Button::new("Record and Transcribe")).clicked() {
            is_recording = false;
        }
    }); 
    // Wait until recording is stopped.
    while is_recording {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // Finalize the WAV file.
    writer.lock().unwrap().take().unwrap().finalize()?;
    println!("Recording {} complete!", file_name);
    Ok(())
}

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = U::from_sample(sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}

