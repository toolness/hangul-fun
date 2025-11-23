use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::{Result, anyhow};
use cpal::traits::{DeviceTrait, HostTrait};

pub fn run_record() -> Result<()> {
    let host = cpal::default_host();
    let Some(device) = host.default_input_device() else {
        return Err(anyhow!("Unable to query default audio input device"));
    };
    if let Ok(name) = device.name() {
        println!("Using device {name:?}.");
    }
    let Ok(supported_configs_range) = device.supported_input_configs() else {
        return Err(anyhow!("Unable to query audio input configs"));
    };
    let mut supported_configs_range =
        supported_configs_range.filter(|range| range.sample_format() == cpal::SampleFormat::F32);
    let Some(config) = supported_configs_range
        .next()
        .map(|range| range.with_max_sample_rate())
    else {
        return Err(anyhow!("Unable to find a supported config"));
    };
    let spec = hound::WavSpec {
        channels: config.channels() as _,
        sample_rate: config.sample_rate().0 as _,
        bits_per_sample: (config.sample_format().sample_size() * 8) as _,
        sample_format: if config.sample_format().is_float() {
            hound::SampleFormat::Float
        } else {
            hound::SampleFormat::Int
        },
    };
    const OUTFILE: &'static str = "recording.wav";
    let writer = hound::WavWriter::create(OUTFILE, spec)?;
    let writer = Arc::new(Mutex::new(Some(writer)));
    let err_fn = move |err| {
        println!("ERROR: {:?}", err);
    };
    let stream_writer = writer.clone();
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &_| {
                if let Ok(mut guard) = stream_writer.try_lock() {
                    if let Some(writer) = guard.as_mut() {
                        for &sample in data.iter() {
                            if let Err(err) = writer.write_sample(sample) {
                                println!("Error writing sample: {err:?}")
                            }
                        }
                    } else {
                        println!("Unable to unwrap mutex!")
                    }
                } else {
                    println!("Unable to lock mutex!")
                }
            },
            err_fn,
            None,
        ),
        _ => {
            return Err(anyhow!(
                "Unsupported sample format: {:?}",
                config.sample_format()
            ));
        }
    };
    let duration = Duration::from_secs(5);
    println!("Recording {duration:?} of audio to {OUTFILE}...");
    std::thread::sleep(duration);
    drop(stream);
    writer.lock().unwrap().take().unwrap().finalize()?;
    println!("Done recording.");
    Ok(())
}
